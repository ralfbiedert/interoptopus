//! Computes trampoline targets for the dotnet reverse-interop pipeline.
//!
//! Each function in the inventory becomes a trampoline entry that describes how
//! an `[UnmanagedCallersOnly]` export should dispatch to managed code:
//!
//! - **Raw** functions forward directly to a static method on `IPlugin`
//!   (e.g. `do_math(a, b) => Plugin.DoMath(a, b)`).
//! - **Service constructors** allocate a new object via the interface's
//!   `static abstract` factory and wrap it in a `GCHandle`.
//! - **Service methods** unwrap the `GCHandle` from the first (`nint self`)
//!   argument and dispatch to the corresponding interface method.
//! - **Service destructors** free the `GCHandle`.

use crate::lang::plugin::{TrampolineEntry, TrampolineKind};
use crate::lang::FunctionId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};
use std::collections::HashMap;

enum ServiceFnRole {
    Ctor,
    Method,
    Destructor,
}

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    entries: Vec<TrampolineEntry>,
    done: bool,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, entries: Vec::new(), done: false }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        fns_all: &model::common::fns::all::Pass,
        service_all: &model::common::service::all::Pass,
    ) -> ModelResult {
        if self.done {
            return Ok(Unchanged);
        }

        // Build a lookup from function id → (service_id, role) for all service-owned functions.
        let mut service_fn_roles: HashMap<FunctionId, (crate::lang::ServiceId, ServiceFnRole)> = HashMap::new();

        for (&svc_id, svc) in service_all.iter() {
            service_fn_roles.insert(svc.destructor, (svc_id, ServiceFnRole::Destructor));
            for &fn_id in &svc.ctors {
                service_fn_roles.insert(fn_id, (svc_id, ServiceFnRole::Ctor));
            }
            for &fn_id in &svc.methods {
                service_fn_roles.insert(fn_id, (svc_id, ServiceFnRole::Method));
            }
        }

        // Check that we have at least some functions available; if not, wait.
        if fns_all.originals().next().is_none() {
            return Ok(Unchanged);
        }

        let mut entries = Vec::new();

        for (&fn_id, _func) in fns_all.originals() {
            let kind = match service_fn_roles.get(&fn_id) {
                Some((svc_id, ServiceFnRole::Ctor)) => TrampolineKind::ServiceCtor { service_id: *svc_id },
                Some((svc_id, ServiceFnRole::Method)) => TrampolineKind::ServiceMethod { service_id: *svc_id },
                Some((svc_id, ServiceFnRole::Destructor)) => TrampolineKind::ServiceDestructor { service_id: *svc_id },
                None => TrampolineKind::Raw,
            };

            entries.push(TrampolineEntry { fn_id, kind });
        }

        self.entries = entries;
        self.done = true;

        let mut outcome = Unchanged;
        outcome.changed();
        Ok(outcome)
    }

    pub fn entries(&self) -> &[TrampolineEntry] {
        &self.entries
    }
}
