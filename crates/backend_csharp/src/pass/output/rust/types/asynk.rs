//! Renders async trampoline classes and their static field declarations.
//!
//! Reads the [`model::rust::types::info::trampoline::Pass`] records to drive
//! emission. All identifier and shape decisions live in that model pass; this
//! output pass is a pure render-and-route layer that:
//!
//! - Iterates trampoline records, gating each into the right output file via
//!   the trampoline's `routing_id` (Result trampolines stay scoped to the
//!   file that owns the Result type; stateless bare trampolines emit into
//!   every file and dedupe by class name).
//! - Renders `templates/rust/types/asynk/trampoline.cs` with a single
//!   `shape` enum string plus the resolved payload, task and class names.

use crate::output::{FileType, Output};
use crate::pass::output::rust::types::asynk_naming;
use crate::pass::{OutputResult, PassInfo, model, output};
use interoptopus_backends::template::Context;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    trampolines: HashMap<Output, Vec<String>>,
    trampoline_fields: HashMap<Output, Vec<String>>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, trampolines: HashMap::default(), trampoline_fields: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::common::master::Pass,
        types: &model::common::types::all::Pass,
        trampoline: &model::rust::types::info::trampoline::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();

        for file in output_master.outputs_of(FileType::Csharp) {
            let mut rendered_trampolines = Vec::new();
            let mut rendered_fields = Vec::new();
            // Bare-Self async ctors all share a single `AsyncTrampolineIntPtr` class
            // (every `*const Service` maps to `IntPtr` in C# and the trampoline body is
            // identical), so dedupe by trampoline class name across distinct TypeIds.
            let mut emitted_trampoline_names: HashSet<String> = HashSet::new();

            for (_, t) in trampoline.iter() {
                // Stateless bare trampolines emit into every output; Result-shaped
                // trampolines route through `type_belongs_to` so they live next to
                // the Result definition.
                if !t.emit_in_every_output && !output_master.type_belongs_to(t.routing_id, file) {
                    continue;
                }

                let names = asynk_naming::names_for(t, types);

                if !emitted_trampoline_names.insert(names.class_name.clone()) {
                    continue;
                }

                let mut context = Context::new();
                context.insert("trampoline_name", &names.class_name);
                context.insert("shape", names.shape.as_str());
                context.insert("payload_full", &names.payload_full);
                context.insert("task_inner_ty", &names.task_inner_name);
                context.insert("is_task_void", &names.is_task_void);
                context.insert("result_to_managed", &names.managed_conversion_method);

                let rendered = templates.render("rust/types/asynk/trampoline.cs", &context)?;
                rendered_trampolines.push(rendered);

                // Render the static field declaration
                let mut field_context = Context::new();
                field_context.insert("trampoline_name", &names.class_name);
                field_context.insert("trampoline_field", &names.field_name);
                let field_rendered = templates.render("rust/types/asynk/trampoline_field.cs", &field_context)?;
                rendered_fields.push(field_rendered);
            }

            rendered_trampolines.sort();
            rendered_fields.sort();

            self.trampolines.insert(file.clone(), rendered_trampolines);
            self.trampoline_fields.insert(file.clone(), rendered_fields);
        }

        Ok(())
    }

    #[must_use]
    pub fn trampolines_for(&self, output: &Output) -> Option<&[String]> {
        self.trampolines.get(output).map(std::vec::Vec::as_slice)
    }

    #[must_use]
    pub fn trampoline_fields_for(&self, output: &Output) -> Option<&[String]> {
        self.trampoline_fields.get(output).map(std::vec::Vec::as_slice)
    }
}
