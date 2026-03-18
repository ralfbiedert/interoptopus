//! Renders the `Bool` pattern type — a blittable `byte`-backed boolean struct.
//!
//! `TypePattern::Bool` maps to this C# `Bool` struct, which avoids SYSLIB1051
//! marshalling issues with native `bool` in `LibraryImport` declarations.
//! Rust's `Primitive::Bool` is also mapped to `TypePattern::Bool` by the
//! primitives model pass, so all booleans funnel through this single type.
//! The struct is emitted in the output file that the `Bool` type is routed to
//! (typically the Common file in multi-file setups).

use crate::lang::types::kind::{TypeKind, TypePattern};
use crate::output::{FileType, Output};
use crate::pass::{OutputResult, PassInfo, model, output};
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

    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta, output_master: &output::master::Pass, types: &model::types::all::Pass) -> OutputResult {
        let templates = output_master.templates();

        for file in output_master.outputs_of(FileType::Csharp) {
            // Only emit Bool into the output file it is routed to.
            let file_has_bool = types.iter().any(|(type_id, ty)| {
                output_master.type_belongs_to(*type_id, file) && matches!(&ty.kind, TypeKind::TypePattern(TypePattern::Bool))
            });

            let content = if file_has_bool {
                templates.render("pattern/bool.cs", &Context::new())?.trim().to_string()
            } else {
                String::new()
            };

            self.rendered.insert(file.clone(), content);
        }

        Ok(())
    }

    #[must_use]
    pub fn bool_for(&self, output: &Output) -> Option<&str> {
        self.rendered.get(output).map(|s| &**s)
    }
}
