//! Wraps composite type definitions through the `all.cs` template, grouped per output file.

use crate::lang::types::TypeKind;
use crate::output::{Output, OutputKind};
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
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, composites: Default::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        kinds: &model::types::kind::Pass,
        composite_ty: &output::types::composites::definition::Pass,
        composite_body: &output::types::composites::body::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for file in output_master.outputs_of(OutputKind::Csharp) {
            let mut rendered_composites = Vec::new();

            for (type_id, type_kind) in kinds.iter() {
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

    pub fn composites_for(&self, output: &Output) -> Option<&[String]> {
        self.composites.get(output).map(|s| s.as_slice())
    }
}
