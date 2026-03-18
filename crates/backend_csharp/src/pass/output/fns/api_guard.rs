//! Renders a static constructor for the `Interop` class that calls the API guard
//! function and checks the returned hash against the one baked into the bindings.
//!
//! If the inventory contains a function whose return type is `TypePattern::ApiVersion`,
//! this pass reads the API hash from `meta::info` and emits a C# static constructor
//! that validates the hash at load time. If no such function exists, the pass produces
//! an empty string and the template simply omits the block.

use crate::lang::FunctionId;
use crate::lang::types::kind::{TypeKind, TypePattern};
use crate::output::{FileType, Output};
use crate::pass::{OutputResult, PassInfo, meta, model, output};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    rendered: HashMap<Output, String>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, rendered: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        fns_all: &model::fns::all::Pass,
        types: &model::types::all::Pass,
        meta_info: &meta::info::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        // Find the API guard function: one whose return type is ApiVersion
        let guard: Option<(FunctionId, String)> = fns_all.originals().find_map(|(&id, function)| {
            let rval_ty = types.get(function.signature.rval)?;
            if matches!(&rval_ty.kind, TypeKind::TypePattern(TypePattern::ApiVersion)) {
                Some((id, function.name.clone()))
            } else {
                None
            }
        });

        for file in output_master.outputs_of(FileType::Csharp) {
            let content = if let Some((fn_id, ref fn_name)) = guard {
                if !output_master.fn_belongs_to(fn_id, file) {
                    String::new()
                } else {
                    let mut context = Context::new();
                    context.insert("fn_name", fn_name);
                    context.insert("hash_hex", meta_info.api_hash());

                    templates.render("fns/api_guard.cs", &context)?.trim().to_string()
                }
            } else {
                String::new()
            };

            self.rendered.insert(file.clone(), content);
        }

        Ok(())
    }

    #[must_use]
    pub fn api_guard_for(&self, output: &Output) -> Option<&str> {
        self.rendered.get(output).map(|s| &**s)
    }
}
