//! Wraps enum type definitions through the `enum.cs` template, grouped per output file.

use crate::lang::types::TypeKind;
use crate::output::{Output, OutputKind};
use crate::pass::{model, output, OutputResult, PassInfo};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    enums: HashMap<Output, Vec<String>>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, enums: Default::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        kinds: &model::types::kind::Pass,
        enum_ty: &output::types::enums::definition::Pass,
        enum_body: &output::types::enums::body::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for file in output_master.outputs_of(OutputKind::Csharp) {
            let mut rendered_enums = Vec::new();

            for (type_id, type_kind) in kinds.iter() {
                match type_kind {
                    TypeKind::DataEnum(_) => {}
                    _ => continue,
                }

                let Some(enum_definition) = enum_ty.get(*type_id) else {
                    continue;
                };

                let Some(body) = enum_body.get(*type_id) else {
                    continue;
                };

                let mut context = Context::new();
                context.insert("enum_definition", enum_definition);
                context.insert("enum_body", body);

                let rendered = templates.render("types/enums/all.cs", &context)?;
                rendered_enums.push(rendered);
            }

            rendered_enums.sort();

            self.enums.insert(file.clone(), rendered_enums);
        }

        Ok(())
    }

    pub fn enums_for(&self, output: &Output) -> Option<&[String]> {
        self.enums.get(output).map(|s| s.as_slice())
    }
}
