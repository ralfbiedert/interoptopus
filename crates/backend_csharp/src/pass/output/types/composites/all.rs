//! Wraps composite type definitions through the `all.cs` template, grouped per output file.

use crate::lang::types::kind::TypeKind;
use crate::output::{FileType, Output};
use crate::pass::{model, output, OutputResult, PassInfo};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    composites: HashMap<Output, Vec<String>>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, composites: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        types: &model::types::all::Pass,
        composite_ty: &output::types::composites::definition::Pass,
        composite_body: &output::types::composites::body::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for file in output_master.outputs_of(FileType::Csharp) {
            let mut rendered_composites = Vec::new();

            for (type_id, ty) in types.iter() {
                if !output_master.type_belongs_to(*type_id, file) {
                    continue;
                }

                let type_kind = &ty.kind;
                match type_kind {
                    TypeKind::Composite(_) => {}
                    _ => continue,
                }

                let Some(composite_definition) = composite_ty.get(*type_id) else {
                    continue;
                };

                let Some(body) = composite_body.get(*type_id) else {
                    continue;
                };

                let mut context = Context::new();
                context.insert("composite_definition", composite_definition);
                context.insert("composite_body", body);

                let rendered = templates.render("types/composite/all.cs", &context)?;
                rendered_composites.push(rendered);
            }

            rendered_composites.sort();

            self.composites.insert(file.clone(), rendered_composites);
        }

        Ok(())
    }

    #[must_use]
    pub fn composites_for(&self, output: &Output) -> Option<&[String]> {
        self.composites.get(output).map(std::vec::Vec::as_slice)
    }
}
