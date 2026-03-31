//! Renders a static constructor for the `Interop` class that calls the API guard
//! function and checks the returned hash against the one baked into the bindings.
//!
//! If the inventory contains a function whose return type is `TypePattern::Version`,
//! this pass reads the API hash from `meta::rust::info` and emits a C# static constructor
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
        output_master: &output::common::master::Pass,
        fns_all: &model::common::fns::all::Pass,
        types: &model::common::types::all::Pass,
        meta_info: &meta::rust::info::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        // Find the API guard function: one whose return type is Version
        let guard: Option<(FunctionId, String)> = fns_all.originals().find_map(|(&id, function)| {
            let rval_ty = types.get(function.signature.rval)?;
            if matches!(&rval_ty.kind, TypeKind::TypePattern(TypePattern::Version)) {
                Some((id, function.name.clone()))
            } else {
                None
            }
        });

        for file in output_master.outputs_of(FileType::Csharp) {
            let content = if let Some((fn_id, ref fn_name)) = guard {
                if output_master.fn_belongs_to(fn_id, file) {
                    let mut context = Context::new();
                    context.insert("fn_name", fn_name);
                    context.insert("hash_hex", meta_info.api_hash());

                    templates.render("rust/fns/guard.cs", &context)?.trim().to_string()
                } else {
                    String::new()
                }
            } else {
                String::new()
            };

            self.rendered.insert(file.clone(), content);
        }

        Ok(())
    }

    #[must_use]
    pub fn guard_for(&self, output: &Output) -> Option<&str> {
        self.rendered.get(output).map(|s| &**s)
    }
}
