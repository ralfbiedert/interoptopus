//! Renders utility types (exceptions, string extensions) per output file.
//!
//! Each utility type has a registered `TypeId` (see `lang::types::csharp`) and
//! participates in dispatch routing. Only utility types routed to a given
//! output file are rendered into that file.

use crate::lang::types::csharp;
use crate::output::{FileType, Output};
use crate::pass::{OutputResult, PassInfo, model, output};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    utils: HashMap<Output, String>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, utils: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::common::master::Pass,
        types: &model::common::types::all::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for file in output_master.outputs_of(FileType::Csharp) {
            let mut parts = Vec::new();

            if output_master.type_belongs_to(csharp::UTIL_INTEROP_EXCEPTION, file) {
                let mut ctx = Context::new();
                ctx.insert("visibility", &visibility_of(csharp::UTIL_INTEROP_EXCEPTION, types));
                parts.push(templates.render("common/types/util/interop_exception.cs", &ctx)?.trim().to_string());
            }
            if output_master.type_belongs_to(csharp::UTIL_ENUM_EXCEPTION, file) {
                let mut ctx = Context::new();
                ctx.insert("visibility", &visibility_of(csharp::UTIL_ENUM_EXCEPTION, types));
                parts.push(templates.render("common/types/util/enum_exception.cs", &ctx)?.trim().to_string());
            }
            if output_master.type_belongs_to(csharp::UTIL_ASYNC_CALLBACK_COMMON, file) {
                let mut ctx = Context::new();
                ctx.insert("visibility", &visibility_of(csharp::UTIL_ASYNC_CALLBACK_COMMON, types));
                parts.push(templates.render("common/types/util/async_callback_common.cs", &ctx)?.trim().to_string());
            }
            if output_master.type_belongs_to(csharp::UTIL_CONST_CSTR_MARSHALLER, file) {
                let mut ctx = Context::new();
                ctx.insert("visibility", &visibility_of(csharp::UTIL_CONST_CSTR_MARSHALLER, types));
                parts.push(templates.render("common/types/util/const_cstr_marshaller.cs", &ctx)?.trim().to_string());
            }
            if output_master.type_belongs_to(csharp::UTIL_TASK_HANDLE, file) {
                let mut ctx = Context::new();
                ctx.insert("visibility", &visibility_of(csharp::UTIL_TASK_HANDLE, types));
                parts.push(templates.render("common/types/util/task_handle.cs", &ctx)?.trim().to_string());
            }

            self.utils.insert(file.clone(), parts.join("\n\n"));
        }
        Ok(())
    }

    #[must_use]
    pub fn utils_for(&self, output: &Output) -> Option<&str> {
        self.utils.get(output).map(|s| &**s)
    }
}

fn visibility_of(type_id: crate::lang::TypeId, types: &model::common::types::all::Pass) -> String {
    types.get(type_id).map_or_else(|| "public".to_string(), |t| t.visibility.to_string())
}
