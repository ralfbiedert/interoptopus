//! Wraps enum type definitions through the `enum.cs` template, grouped per output file.

use crate::lang::types::kind::{TypeKind, TypePattern};
use crate::output::{Output, OutputKind};
use crate::pass::{OutputResult, PassInfo, model, output};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    enums: HashMap<Output, Vec<String>>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, enums: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::master::Pass,
        types: &model::types::all::Pass,
        enum_ty: &output::types::enums::definition::Pass,
        enum_body: &output::types::enums::body::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for file in output_master.outputs_of(OutputKind::Csharp) {
            let mut rendered_enums = Vec::new();

            for (type_id, ty) in types.iter() {
                let type_kind = &ty.kind;
                match type_kind {
                    TypeKind::DataEnum(_) => {}
                    TypeKind::TypePattern(TypePattern::Result(_, _, e)) => {}
                    TypeKind::TypePattern(TypePattern::Option(_, e)) => {}
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

    #[must_use]
    pub fn enums_for(&self, output: &Output) -> Option<&[String]> {
        self.enums.get(output).map(std::vec::Vec::as_slice)
    }
}
