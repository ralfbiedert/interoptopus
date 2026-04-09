//! Emits managed C# classes for nested structs reachable from `Wire<T>` inner types.
//!
//! These are types registered as `TypeKind::WireOnly(WireOnly::Composite(...))`
//! by the model wire pass. This output pass renders them as simple `partial class`
//! declarations with public fields.

use crate::lang::types::kind::TypeKind;
use crate::lang::types::kind::wire::WireOnly;
use crate::output::{FileType, Output};
use crate::pass::output::common::wire::WireCodeGen;
use crate::pass::{OutputResult, PassInfo, model, output};
use interoptopus::inventory::Types as RsTypes;
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    helper_classes: HashMap<Output, Vec<String>>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, helper_classes: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::common::master::Pass,
        types: &model::common::types::all::Pass,
        id_map: &model::common::id_map::Pass,
        rs_types: &RsTypes,
        wire_types: &output::common::wire::wire_type::Pass,
    ) -> OutputResult {
        let templates = output_master.templates();
        let codegen = WireCodeGen { rs_types };

        // Route each helper class to the output file its type is routed to.
        let mut helpers_by_output: HashMap<Output, Vec<String>> = HashMap::new();

        for (type_id, ty) in types.iter() {
            let TypeKind::WireOnly(WireOnly::Composite(composite)) = &ty.kind else {
                continue;
            };

            // Skip types whose field definitions are already rendered by
            // the wire_type pass as part of a Wire<T> block.
            if wire_types.rendered_inner_type(*type_id) {
                continue;
            }

            // Resolve field type names. Fields may reference WireOnly types
            // (String, Vec, Map) that don't exist in the C# type model, so
            // we resolve names via the Rust type graph.
            let field_decls: Vec<String> = composite
                .fields
                .iter()
                .map(|f| {
                    let field_type_name = resolve_field_type_name(f.ty, types, id_map, &codegen);
                    format!("public {field_type_name} {};", f.name)
                })
                .collect();

            let mut ctx = Context::new();
            ctx.insert("class_name", &ty.name);
            ctx.insert("field_decls", &field_decls);

            let result = templates.render("common/wire/wire_helper_class.cs", &ctx)?;

            // Find the output file this type is routed to.
            let target_output = output_master
                .outputs_of(FileType::Csharp)
                .find(|o| output_master.type_belongs_to(*type_id, o))
                .cloned();

            if let Some(output) = target_output {
                helpers_by_output.entry(output).or_default().push(result);
            } else {
                // Fall back to first output file if no routing exists.
                if let Some(first) = output_master.outputs_of(FileType::Csharp).next() {
                    helpers_by_output.entry(first.clone()).or_default().push(result);
                }
            }
        }

        // Sort helpers within each output for deterministic output.
        for helpers in helpers_by_output.values_mut() {
            helpers.sort();
        }

        // Ensure all output files have an entry (even if empty).
        for file in output_master.outputs_of(FileType::Csharp) {
            self.helper_classes.entry(file.clone()).or_default();
        }
        self.helper_classes.extend(helpers_by_output);

        Ok(())
    }

    #[must_use]
    pub fn helper_classes_for(&self, output: &Output) -> Option<&[String]> {
        self.helper_classes.get(output).map(std::vec::Vec::as_slice)
    }
}

/// Resolves the C# type name for a field of a wire helper class.
///
/// Wire helper classes represent managed C# types, so their fields should use
/// managed type names (`string`, `List<T>`, `uint?`, etc.) rather than FFI
/// envelope names (`Utf8String`, `VecU8`, `OptionUint`).  We always prefer the
/// wire codegen which produces the correct managed C# type name.
fn resolve_field_type_name(
    cs_ty: crate::lang::TypeId,
    types: &model::common::types::all::Pass,
    id_map: &model::common::id_map::Pass,
    codegen: &WireCodeGen<'_>,
) -> String {
    for rs_id in codegen.rs_types.keys() {
        if id_map.ty(*rs_id) == Some(cs_ty) {
            return codegen.cs_type_name(*rs_id);
        }
    }

    if let Some(t) = types.get(cs_ty) {
        return t.name.clone();
    }
    "object".to_string()
}
