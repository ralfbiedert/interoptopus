//! Adds overload function IDs to service ctors and methods.
//!
//! For each service ctor/method that is an original function, this pass checks
//! whether the underlying function has any overloads in `fns::all`. It inserts
//! those overload `FunctionId`s right after the original in the service's `ctors`
//! and `methods` lists, so that output passes can simply iterate over these lists
//! without querying overload registries.
//!
//! This pass also handles conflict detection: for non-async overloads, if stripping
//! the self-arg produces the same signature as the base method, the overload is
//! skipped (since C# would see duplicate method signatures).

use crate::lang::FunctionId;
use crate::lang::functions::FunctionKind;
use crate::lang::functions::overload::OverloadKind;
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo, model};

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() } }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        services: &mut model::service::all::Pass,
        fns_all: &model::fns::all::Pass,
        types: &model::types::all::Pass,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        // Collect rebuilt lists, then apply
        let mut method_rebuilds: Vec<(crate::lang::ServiceId, Vec<FunctionId>)> = Vec::new();

        for (&service_id, service) in services.iter() {
            // Rebuild methods with overloads inserted after their originals
            let new_methods = rebuild_with_overloads(&service.methods, fns_all, types);
            if let Some(new_methods) = new_methods {
                method_rebuilds.push((service_id, new_methods));
            }
        }

        // Apply rebuilt lists
        for (service_id, new_methods) in method_rebuilds {
            if let Some(service) = services.get_mut(service_id) {
                service.methods = new_methods;
                outcome.changed();
            }
        }

        Ok(outcome)
    }
}

/// Rebuild a function list by inserting overloads right after their base original.
/// Returns `None` if no changes were made (no new overloads to add).
fn rebuild_with_overloads(fn_ids: &[FunctionId], fns_all: &model::fns::all::Pass, types: &model::types::all::Pass) -> Option<Vec<FunctionId>> {
    let mut result = Vec::new();
    let mut changed = false;

    for &fn_id in fn_ids {
        result.push(fn_id);

        // Only look for overloads of original functions
        let Some(func) = fns_all.get(fn_id) else { continue };
        if !matches!(func.kind, FunctionKind::Original) {
            continue;
        }

        for (overload_id, overload_fn) in fns_all.overloads_for(fn_id) {
            // Skip if already in the list
            if fn_ids.contains(overload_id) {
                continue;
            }

            // For non-async overloads, check if stripping the self-arg
            // would produce a duplicate signature. If so, skip it.
            let dominated = match &overload_fn.kind {
                FunctionKind::Overload(o) if !matches!(o.kind, OverloadKind::Async(_)) => {
                    service_args_equivalent(&func.signature.arguments, &overload_fn.signature.arguments, types)
                }
                _ => false,
            };
            if dominated {
                continue;
            }

            result.push(*overload_id);
            changed = true;
        }
    }

    if changed { Some(result) } else { None }
}

/// Checks if two function signatures are equivalent after stripping the self-arg
/// (first argument). This is used to detect overloads that would produce duplicate
/// service methods in C#.
fn service_args_equivalent(base_args: &[crate::lang::functions::Argument], overload_args: &[crate::lang::functions::Argument], types: &model::types::all::Pass) -> bool {
    let base_rest = if base_args.len() > 1 { &base_args[1..] } else { &[] };
    let overload_rest = if overload_args.len() > 1 { &overload_args[1..] } else { &[] };

    if base_rest.len() != overload_rest.len() {
        return false;
    }

    base_rest.iter().zip(overload_rest.iter()).all(|(a, b)| {
        let a_name = types.get(a.ty).map(|t| &t.name);
        let b_name = types.get(b.ty).map(|t| &t.name);
        a.name == b.name && a_name == b_name
    })
}
