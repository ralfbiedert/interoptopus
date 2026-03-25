//! Renders partial class definitions for service types.
//!
//! Each `TypeKind::Service` type gets a partial class with an inner `Unmanaged`
//! struct (wrapping `IntPtr`) and conversion methods (`IntoManaged`, `AsManaged`,
//! `IntoUnmanaged`, `AsUnmanaged`, `Dispose`). This makes service types participate
//! in the same managed/unmanaged conversion pattern as composites and enums.

use crate::lang::types::kind::TypeKind;
use crate::output::{FileType, Output};
use crate::pass::{OutputResult, PassInfo, model, output};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    services: HashMap<Output, Vec<String>>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, services: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::common::master::Pass,
        types: &model::common::types::all::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for file in output_master.outputs_of(FileType::Csharp) {
            let mut rendered = Vec::new();

            for (type_id, ty) in types.iter() {
                if !matches!(&ty.kind, TypeKind::Service) {
                    continue;
                }
                if !output_master.type_belongs_to(*type_id, file) {
                    continue;
                }

                let mut ctx = Context::new();
                ctx.insert("name", &ty.name);

                let text = templates.render("common/types/service/all.cs", &ctx)?;
                rendered.push(text);
            }

            rendered.sort();
            self.services.insert(file.clone(), rendered);
        }

        Ok(())
    }

    #[must_use]
    pub fn services_for(&self, output: &Output) -> Option<&[String]> {
        self.services.get(output).map(Vec::as_slice)
    }
}
