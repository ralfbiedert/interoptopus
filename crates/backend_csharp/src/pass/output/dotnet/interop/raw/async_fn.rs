//! Renders `[UnmanagedCallersOnly]` trampoline methods for asynchronous raw (non-service)
//! functions.
//!
//! Async functions have an `AsyncCallback<T>` as their last argument. The generated
//! trampoline strips the callback from forwarded args and attaches a
//! `.ContinueWith(...)` continuation that completes the `AsyncCallback`.
//!
//! Functions returning service types use `.IntoUnmanaged()` in the continuation.

use crate::lang::FunctionId;
use crate::lang::plugin::TrampolineKind;
use crate::pass::output::dotnet::interop::raw::resolve_ptr_to_service_name;
use crate::pass::output::dotnet::interop::{async_callback_inner, async_continuation, unmanaged_args_except_last};
use crate::pass::{OutputResult, PassInfo, model, output};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    methods: HashMap<FunctionId, String>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, methods: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::common::master::Pass,
        trampoline_model: &model::dotnet::trampoline::Pass,
        plugin_interface: &model::dotnet::interface::plugin::Pass,
        fns_all: &model::common::fns::all::Pass,
        types: &model::common::types::all::Pass,
        unmanaged_names: &output::common::conversion::unmanaged_names::Pass,
        unmanaged_conversion: &output::common::conversion::unmanaged_conversion::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        let method_names: HashMap<FunctionId, &str> = plugin_interface
            .interface()
            .map(|iface| iface.methods.iter().map(|m| (m.base, m.name.as_str())).collect())
            .unwrap_or_default();

        let result_wraps: HashMap<FunctionId, &str> = plugin_interface
            .interface()
            .map(|iface| {
                iface
                    .methods
                    .iter()
                    .filter_map(|m| {
                        let result_id = m.unwrapped_result_id?;
                        let name = types.get(result_id).map(|t| t.name.as_str())?;
                        Some((m.base, name))
                    })
                    .collect()
            })
            .unwrap_or_default();

        for entry in trampoline_model.entries() {
            if !matches!(entry.kind, TrampolineKind::Raw) {
                continue;
            }

            let Some(func) = fns_all.get(entry.fn_id) else { continue };
            let Some(&pascal_name) = method_names.get(&entry.fn_id) else { continue };

            // Only handle async raw functions.
            let Some(inner_id) = async_callback_inner(func, types) else { continue };

            let ffi_name = &func.name;
            let result_wrap_type = result_wraps.get(&entry.fn_id).copied().unwrap_or("");

            let (args, forward) = unmanaged_args_except_last(func, unmanaged_names, unmanaged_conversion);

            // Async service return: use .IntoUnmanaged() in continuation.
            let continuation = if resolve_ptr_to_service_name(inner_id, types).is_some() {
                "ContinueWith(t => cb.UnsafeComplete(t.Result.IntoUnmanaged()))".to_string()
            } else {
                async_continuation(inner_id, types, unmanaged_conversion)
            };

            let mut ctx = Context::new();
            ctx.insert("ffi_name", ffi_name);
            ctx.insert("args", &args);
            ctx.insert("pascal_name", pascal_name);
            ctx.insert("forward", &forward);
            ctx.insert("continuation", &continuation);
            ctx.insert("result_wrap_type", result_wrap_type);
            let rendered = templates.render("dotnet/interop/raw_async.cs", &ctx)?;

            self.methods.insert(entry.fn_id, rendered.trim_end().to_string());
        }

        Ok(())
    }

    #[must_use]
    pub fn get(&self, fn_id: FunctionId) -> Option<&str> {
        self.methods.get(&fn_id).map(String::as_str)
    }
}
