//! Renders `FromCall` and `FromCallAsync` helper methods for Result types.
//!
//! These static methods wrap a user-provided lambda in a try/catch block and
//! convert exceptions into the appropriate Result variant:
//!
//! - For `Result<T, DotnetException>` ("try error"): exceptions map to `Err(…)`.
//! - For any other `Result<T, E>`: exceptions map to `Panic`.
//!
//! ```csharp
//! public static ResultFooError FromCall(Func<Foo> func)
//! {
//!     try { return Ok(func()); }
//!     catch (Exception) { return Panic; }
//! }
//! ```

use crate::lang::TypeId;
use crate::lang::types::kind::{TypeKind, TypePattern};
use crate::pass::{OutputResult, PassInfo, model, output};
use crate::pattern::ExceptionError;
use interoptopus::lang::types::TypeInfo;
use interoptopus_backends::template::{Context, Value};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    body_from_call: HashMap<TypeId, String>,
}

impl Pass {
    #[must_use]
    pub fn new(config: Config) -> Self {
        Self { info: PassInfo { name: file!() }, body_from_call: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::common::master::Pass,
        types: &model::common::types::all::Pass,
        id_maps: &model::common::id_map::Pass,
        exceptions_model: &model::common::exceptions::Pass,
        mode: crate::pass::OperationMode,
    ) -> OutputResult {
        let templates = output_master.templates();
        let try_error_id = { id_maps.ty(ExceptionError::id()) };

        let exceptions: Vec<HashMap<&str, Value>> = exceptions_model
            .exceptions()
            .iter()
            .map(|e| {
                let mut m = HashMap::new();
                m.insert("name", Value::String(e.name().to_string()));
                m.insert("id", Value::String(format!("0x{:X}UL", e.id())));
                m
            })
            .collect();

        for (type_id, ty) in types.iter() {
            let (ok_id, err_id) = match &ty.kind {
                TypeKind::TypePattern(TypePattern::Result(ok, err, _)) => (*ok, *err),
                _ => continue,
            };

            let name = &ty.name;

            // In Plugin mode, resolve pointer-to-service so the Ok type
            // shows the managed service class name instead of IntPtr.
            let resolved_ok = if mode == crate::pass::OperationMode::Plugin {
                super::resolve_service_variant(ok_id, types, mode)
            } else {
                ok_id
            };

            let ok_has_payload = !matches!(types.get(resolved_ok).map(|t| &t.kind), Some(TypeKind::Primitive(crate::lang::types::kind::Primitive::Void)));

            let ok_type = if ok_has_payload {
                types.get(resolved_ok).map_or("void", |t| t.name.as_str()).to_string()
            } else {
                String::new()
            };

            let is_try_error = try_error_id.is_some_and(|id| err_id == id);
            let err_type = if is_try_error {
                types.get(err_id).map_or("", |t| t.name.as_str()).to_string()
            } else {
                String::new()
            };

            let mut ctx = Context::new();
            ctx.insert("name", name);
            ctx.insert("ok_type", &ok_type);
            ctx.insert("ok_has_payload", &ok_has_payload);
            ctx.insert("is_try_error", &is_try_error);
            ctx.insert("err_type", &err_type);
            ctx.insert("exceptions", &exceptions);

            let rendered = templates.render("common/types/enums/body_from_call.cs", &ctx)?;
            self.body_from_call.insert(*type_id, rendered);
        }

        Ok(())
    }

    #[must_use]
    pub fn get(&self, type_id: TypeId) -> Option<&String> {
        self.body_from_call.get(&type_id)
    }
}
