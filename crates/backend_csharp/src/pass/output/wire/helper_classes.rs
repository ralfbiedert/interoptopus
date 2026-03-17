//! Emits managed C# classes for nested structs reachable from `Wire<T>` inner types.
//!
//! These are types registered as `TypeKind::WireOnly(WireOnly::Composite(...))`
//! by the model wire pass. This output pass renders them as simple `partial class`
//! declarations with public fields.

use crate::lang::types::kind::wire::WireOnly;
use crate::lang::types::kind::TypeKind;
use crate::output::{FileType, Output};
use crate::pass::{model, output, OutputResult, PassInfo};
use interoptopus::inventory::Types as RsTypes;
use interoptopus_backends::template::Context;
use std::collections::HashMap;
use crate::pass::output::wire::WireCodeGen;

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
        output_master: &output::master::Pass,
        types: &model::types::all::Pass,
        id_map: &model::id_map::Pass,
        rs_types: &RsTypes,
    ) -> OutputResult {
        let templates = output_master.templates();
        let codegen = WireCodeGen { rs_types };

        for file in output_master.outputs_of(FileType::Csharp) {
            let mut rendered = Vec::new();

            for (_type_id, ty) in types.iter() {
                let TypeKind::WireOnly(WireOnly::Composite(composite)) = &ty.kind else {
                    continue;
                };

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

                let result = templates.render("wire/wire_helper_class.cs", &ctx)?;
                rendered.push(result);
            }

            rendered.sort();
            self.helper_classes.insert(file.clone(), rendered);
        }

        Ok(())
    }

    #[must_use]
    pub fn helper_classes_for(&self, output: &Output) -> Option<&[String]> {
        self.helper_classes.get(output).map(std::vec::Vec::as_slice)
    }
}

/// Resolves the C# type name for a field. Tries the C# type model first,
/// falls back to the Rust-based wire codegen for WireOnly types.
fn resolve_field_type_name(cs_ty: crate::lang::TypeId, types: &model::types::all::Pass, id_map: &model::id_map::Pass, codegen: &WireCodeGen<'_>) -> String {
    // Try C# type model first.
    if let Some(t) = types.get(cs_ty) {
        return t.name.clone();
    }
    // Fall back: find the Rust TypeId and resolve via wire codegen.
    for (rs_id, _) in codegen.rs_types {
        if id_map.ty(*rs_id) == Some(cs_ty) {
            return codegen.cs_type_name(*rs_id);
        }
    }
    "object".to_string()
}
