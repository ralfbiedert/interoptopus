//! Renders `[UnmanagedCallersOnly]` trampoline methods for raw (non-service) functions.
//!
//! Each `TrampolineKind::Raw` entry produces one method inside `public static class Interop`.
//! Sync functions forward to `Plugin.Method(args…)`, while async functions attach a
//! `.ContinueWith(...)` continuation that completes the `AsyncCallback`.

use crate::lang::FunctionId;
use crate::lang::plugin::TrampolineKind;
use crate::pass::{OutputResult, PassInfo, model, output};
use crate::pass::output::dotnet::interop::{async_callback_inner, async_continuation, rval_unmanaged_name, unmanaged_args, unmanaged_args_except_last};
use interoptopus_backends::casing::rust_to_pascal;
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
        fns_all: &model::common::fns::all::Pass,
        types: &model::common::types::all::Pass,
        unmanaged_names: &output::common::conversion::unmanaged_names::Pass,
        unmanaged_conversion: &output::common::conversion::unmanaged_conversion::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for entry in trampoline_model.entries() {
            if !matches!(entry.kind, TrampolineKind::Raw) {
                continue;
            }

            let Some(func) = fns_all.get(entry.fn_id) else { continue };

            let ffi_name = &func.name;
            let pascal_name = rust_to_pascal(ffi_name);
            let rval_type = types.get(func.signature.rval).map_or("void", |t| t.name.as_str());
            let async_inner = async_callback_inner(func, types);

            let rendered = if let Some(inner_id) = async_inner {
                let (args, forward) = unmanaged_args_except_last(func, unmanaged_names, unmanaged_conversion);
                let continuation = async_continuation(inner_id, types, unmanaged_conversion);

                let mut ctx = Context::new();
                ctx.insert("ffi_name", ffi_name);
                ctx.insert("args", &args);
                ctx.insert("pascal_name", &pascal_name);
                ctx.insert("forward", &forward);
                ctx.insert("continuation", &continuation);
                templates.render("dotnet/interop/raw_async.cs", &ctx)?
            } else {
                let (args, forward) = unmanaged_args(func, unmanaged_names, unmanaged_conversion);
                let rval_unmanaged = rval_unmanaged_name(func, rval_type, unmanaged_names);
                let rval_suffix = unmanaged_conversion.to_unmanaged_suffix(func.signature.rval);
                let is_void = rval_type == "void";

                let mut ctx = Context::new();
                ctx.insert("ffi_name", ffi_name);
                ctx.insert("rval_type", rval_unmanaged);
                ctx.insert("args", &args);
                ctx.insert("pascal_name", &pascal_name);
                ctx.insert("forward", &forward);
                ctx.insert("rval_suffix", &rval_suffix);
                ctx.insert("is_void", &is_void);
                templates.render("dotnet/interop/raw_sync.cs", &ctx)?
            };

            self.methods.insert(entry.fn_id, rendered.trim_end().to_string());
        }

        Ok(())
    }

    #[must_use]
    pub fn get(&self, fn_id: FunctionId) -> Option<&str> {
        self.methods.get(&fn_id).map(String::as_str)
    }
}
