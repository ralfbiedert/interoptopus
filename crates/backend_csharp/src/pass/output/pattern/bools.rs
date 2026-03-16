//! Renders the `Bool` pattern type — a blittable `byte`-backed boolean struct.
//!
//! Both `Primitive::Bool` and `TypePattern::Bool` map to this C# `Bool` struct,
//! which avoids SYSLIB1051 marshalling issues with native `bool` in `LibraryImport`
//! declarations. The struct is always emitted once per output file that contains
//! any type mapping to `Bool`.

use crate::lang::types::kind::{Primitive, TypeKind, TypePattern};
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

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        types: &model::types::all::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        // Check if any type in the model is a Bool (primitive or pattern).
        let has_bool = types.iter().any(|(_, ty)| {
            matches!(&ty.kind, TypeKind::Primitive(Primitive::Bool) | TypeKind::TypePattern(TypePattern::Bool))
        });

        for file in output_master.outputs_of(FileType::Csharp) {
            let content = if has_bool {
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
