//! Renders `WireOf*` structs for each `Wire<T>` pattern type per output file.
//!
//! For each `Wire<T>` in the Rust inventory, emits a C# `WireOfT` struct with
//! `From()`, `Unwire()`, `CalculateSize()`, and `Dispose()` methods. When the
//! inner type is a struct, also emits a managed class with a `.Wire()` helper.

use crate::lang::TypeId;
use crate::output::{FileType, Output};
use crate::pass::output::common::wire::WireCodeGen;
use crate::pass::{OutputResult, PassInfo, model, output};
use interoptopus::inventory::Types as RsTypes;
use interoptopus::lang::types::TypeKind as RsTypeKind;
use interoptopus_backends::template::Context;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    wire_types: HashMap<Output, Vec<String>>,
    /// C# `TypeIds` of `Wire<T>` inner types whose field definitions were rendered
    /// by this pass. The `helper_classes` pass checks this to avoid duplicates.
    rendered_inner_types: HashSet<TypeId>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, wire_types: HashMap::default(), rendered_inner_types: HashSet::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::common::master::Pass,
        types: &model::common::types::all::Pass,
        id_map: &model::common::id_map::Pass,
        rs_types: &RsTypes,
    ) -> OutputResult {
        let templates = output_master.templates();
        let codegen = WireCodeGen { rs_types };

        for file in output_master.outputs_of(FileType::Csharp) {
            let mut rendered = Vec::new();

            for (rust_id, rust_ty) in rs_types {
                let inner_rust_id = match &rust_ty.kind {
                    RsTypeKind::TypePattern(interoptopus::lang::types::TypePattern::Wire(inner)) => *inner,
                    _ => continue,
                };

                let Some(cs_wire_id) = id_map.ty(*rust_id) else { continue };
                if !output_master.type_belongs_to(cs_wire_id, file) {
                    continue;
                }

                let Some(cs_wire_ty) = types.get(cs_wire_id) else { continue };
                let wire_name = &cs_wire_ty.name;

                let Some(inner_rust_ty) = rs_types.get(&inner_rust_id) else { continue };
                let inner_name = codegen.cs_type_name(inner_rust_id);

                // Track that this inner type's fields are rendered here, so the
                // helper_classes pass can skip it.
                if let Some(cs_inner_id) = id_map.ty(inner_rust_id) {
                    self.rendered_inner_types.insert(cs_inner_id);
                }

                // If the inner type already has TypeKind::Composite, the composites
                // pass renders its field definitions — emit only the Wire() method.
                let inner_is_composite = id_map
                    .ty(inner_rust_id)
                    .and_then(|cs_id| types.get(cs_id))
                    .is_some_and(|ty| matches!(&ty.kind, crate::lang::types::kind::TypeKind::Composite(_)));

                let (has_managed_class, field_decls, serialize_body, deserialize_body, size_body) = if let RsTypeKind::Struct(s) = &inner_rust_ty.kind {
                    let fields: Vec<String> = if inner_is_composite {
                        vec![]
                    } else {
                        s.fields.iter().map(|f| format!("public {} {};", codegen.cs_type_name(f.ty), f.name)).collect()
                    };
                    (true, fields, codegen.serialize_struct_body(s, "value"), codegen.deserialize_struct_body(s, &inner_name), codegen.size_struct_body(s, "value"))
                } else {
                    let mut ser = Vec::new();
                    codegen.emit_serialize(&mut ser, inner_rust_id, "value", 0, 0);

                    let mut deser = Vec::new();
                    deser.push(format!("{inner_name} result = default;"));
                    codegen.emit_deserialize(&mut deser, inner_rust_id, "result", 0, 0);
                    deser.push("return result;".to_string());

                    let mut size_lines = Vec::new();
                    size_lines.push("var _size = 0;".to_string());
                    codegen.emit_size(&mut size_lines, inner_rust_id, "value", 0, 0);
                    size_lines.push("return _size;".to_string());

                    (false, vec![], ser.join("\n"), deser.join("\n"), size_lines.join("\n"))
                };

                let mut context = Context::new();
                context.insert("wire_name", wire_name);
                context.insert("inner_type", &inner_name);
                context.insert("has_managed_class", &has_managed_class);
                context.insert("field_decls", &field_decls);
                context.insert("serialize_body", &serialize_body);
                context.insert("deserialize_body", &deserialize_body);
                context.insert("size_body", &size_body);

                let result = templates.render("common/wire/wire_type.cs", &context)?;
                rendered.push(result);
            }

            rendered.sort();
            self.wire_types.insert(file.clone(), rendered);
        }

        Ok(())
    }

    #[must_use]
    pub fn wire_types_for(&self, output: &Output) -> Option<&[String]> {
        self.wire_types.get(output).map(std::vec::Vec::as_slice)
    }

    /// Returns true if the given C# `TypeId` is a Wire<T> inner type whose field
    /// definitions were already rendered by this pass.
    #[must_use]
    pub fn rendered_inner_type(&self, type_id: TypeId) -> bool {
        self.rendered_inner_types.contains(&type_id)
    }
}
