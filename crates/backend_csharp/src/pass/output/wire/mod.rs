//! Renders `Wire<T>` counterpart types (`WireOf*`) per output file.
//!
//! For each `Wire<T>` pattern type, emits a `WireOfT` struct with inline
//! serialization, deserialization, and size calculation logic. The code
//! generation walks nested composites recursively so the inner type `T`
//! does not need any serde methods of its own.

use crate::lang::types::kind::{TypeKind, TypePattern};
use crate::output::{FileType, Output};
use crate::pass::{OutputResult, PassInfo, model, output};
use interoptopus::inventory::Types as RsTypes;
use interoptopus::lang::types::{Primitive, Struct, TypeKind as RsTypeKind, WireOnly};
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    wires: HashMap<Output, Vec<String>>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, wires: HashMap::default() }
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
            let mut rendered_wires = Vec::new();

            // Iterate Rust types to find Wire<T> patterns.
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

                // Resolve the inner Rust type.
                let Some(inner_rust_ty) = rs_types.get(&inner_rust_id) else { continue };
                let inner_name = codegen.cs_type_name(inner_rust_id);

                // For structs, generate a managed class with fields and .Wire() helper.
                // For other types (primitives, strings, etc.) the C# type already exists.
                let (has_managed_class, field_decls, serialize_body, deserialize_body, size_body) =
                    if let RsTypeKind::Struct(s) = &inner_rust_ty.kind {
                        let fields: Vec<String> = s.fields.iter()
                            .map(|f| format!("public {} {};", codegen.cs_type_name(f.ty), f.name))
                            .collect();
                        (
                            true,
                            fields,
                            codegen.serialize_struct_body(s, "value"),
                            codegen.deserialize_struct_body(s, &inner_name),
                            codegen.size_struct_body(s, "value"),
                        )
                    } else {
                        let mut ser = Vec::new();
                        codegen.emit_serialize(&mut ser, inner_rust_id, "value", 0);

                        let mut deser = Vec::new();
                        deser.push(format!("{inner_name} result = default;"));
                        codegen.emit_deserialize(&mut deser, inner_rust_id, "result", 0);
                        deser.push("return result;".to_string());

                        let mut size_lines = Vec::new();
                        size_lines.push("var _size = 0;".to_string());
                        codegen.emit_size(&mut size_lines, inner_rust_id, "value", 0);
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

                let rendered = templates.render("wire/wire_type.cs", &context)?;
                rendered_wires.push(rendered);
            }

            rendered_wires.sort();
            self.wires.insert(file.clone(), rendered_wires);
        }

        Ok(())
    }

    #[must_use]
    pub fn wires_for(&self, output: &Output) -> Option<&[String]> {
        self.wires.get(output).map(std::vec::Vec::as_slice)
    }
}

/// Generates C# serialization code for the wire format by walking Rust types.
struct WireCodeGen<'a> {
    rs_types: &'a RsTypes,
}

impl WireCodeGen<'_> {
    /// Maps a Rust type to its C# managed type name.
    fn cs_type_name(&self, ty_id: interoptopus::inventory::TypeId) -> String {
        let Some(ty) = self.rs_types.get(&ty_id) else {
            return "object".to_string();
        };
        match &ty.kind {
            RsTypeKind::Primitive(p) => cs_primitive_name(*p).to_string(),
            RsTypeKind::WireOnly(WireOnly::String) => "string".to_string(),
            RsTypeKind::WireOnly(WireOnly::Vec(inner)) => {
                format!("List<{}>", self.cs_type_name(*inner))
            }
            RsTypeKind::WireOnly(WireOnly::Map(k, v)) => {
                format!("Dictionary<{}, {}>", self.cs_type_name(*k), self.cs_type_name(*v))
            }
            RsTypeKind::Struct(_) => ty.name.clone(),
            _ => "object".to_string(),
        }
    }

    /// Generates the serialize method body for a struct.
    fn serialize_struct_body(&self, s: &Struct, val: &str) -> String {
        let mut lines = Vec::new();
        for f in &s.fields {
            let access = format!("{val}.{}", f.name);
            self.emit_serialize(&mut lines, f.ty, &access, 0);
        }
        lines.join("\n")
    }

    /// Generates the deserialize method body for a struct.
    fn deserialize_struct_body(&self, s: &Struct, type_name: &str) -> String {
        let mut lines = Vec::new();
        lines.push(format!("var result = new {type_name}();"));
        for f in &s.fields {
            let target = format!("result.{}", f.name);
            self.emit_deserialize(&mut lines, f.ty, &target, 0);
        }
        lines.push("return result;".to_string());
        lines.join("\n")
    }

    /// Generates the size calculation body for a struct.
    fn size_struct_body(&self, s: &Struct, val: &str) -> String {
        let mut lines = Vec::new();
        lines.push("var _size = 0;".to_string());
        for f in &s.fields {
            let access = format!("{val}.{}", f.name);
            self.emit_size(&mut lines, f.ty, &access, 0);
        }
        lines.push("return _size;".to_string());
        lines.join("\n")
    }

    /// Emits C# statements to serialize a value of the given Rust type.
    fn emit_serialize(&self, lines: &mut Vec<String>, ty_id: interoptopus::inventory::TypeId, val: &str, depth: usize) {
        let Some(ty) = self.rs_types.get(&ty_id) else { return };
        match &ty.kind {
            RsTypeKind::Primitive(p) => {
                if *p == Primitive::Bool {
                    lines.push(format!("writer.Write({val} ? (byte)1 : (byte)0);"));
                } else {
                    lines.push(format!("writer.Write({val});"));
                }
            }
            RsTypeKind::WireOnly(WireOnly::String) => {
                lines.push(format!("{{ var _bytes = System.Text.Encoding.UTF8.GetBytes({val} ?? \"\"); writer.Write((uint)_bytes.Length); writer.Write(_bytes); }}"));
            }
            RsTypeKind::WireOnly(WireOnly::Vec(inner_id)) => {
                let iter = format!("_item{depth}");
                lines.push(format!("writer.Write((uint)({val}?.Count ?? 0));"));
                lines.push(format!("if ({val} != null) {{ foreach (var {iter} in {val}) {{"));
                self.emit_serialize(lines, *inner_id, &iter, depth + 1);
                lines.push("} }".to_string());
            }
            RsTypeKind::WireOnly(WireOnly::Map(k_id, v_id)) => {
                let kv = format!("_kv{depth}");
                lines.push(format!("writer.Write((uint)({val}?.Count ?? 0));"));
                lines.push(format!("if ({val} != null) {{ foreach (var {kv} in {val}) {{"));
                self.emit_serialize(lines, *k_id, &format!("{kv}.Key"), depth + 1);
                self.emit_serialize(lines, *v_id, &format!("{kv}.Value"), depth + 1);
                lines.push("} }".to_string());
            }
            RsTypeKind::Struct(s) => {
                for f in &s.fields {
                    self.emit_serialize(lines, f.ty, &format!("{val}.{}", f.name), depth);
                }
            }
            _ => {
                lines.push(format!("/* unsupported wire type for {val} */"));
            }
        }
    }

    /// Emits C# statements to deserialize a value and assign it to `target`.
    fn emit_deserialize(&self, lines: &mut Vec<String>, ty_id: interoptopus::inventory::TypeId, target: &str, depth: usize) {
        let Some(ty) = self.rs_types.get(&ty_id) else { return };
        match &ty.kind {
            RsTypeKind::Primitive(p) => {
                lines.push(format!("{target} = {};", cs_read_primitive(*p)));
            }
            RsTypeKind::WireOnly(WireOnly::String) => {
                lines.push(format!(
                    "{{ var _len = reader.ReadUInt32(); {target} = _len > 0 ? System.Text.Encoding.UTF8.GetString(reader.ReadBytes((int)_len)) : \"\"; }}"
                ));
            }
            RsTypeKind::WireOnly(WireOnly::Vec(inner_id)) => {
                let cs_inner = self.cs_type_name(*inner_id);
                let count = format!("_count{depth}");
                let idx = format!("_i{depth}");
                lines.push(format!("{{ var {count} = reader.ReadUInt32(); {target} = new List<{cs_inner}>((int){count});"));
                lines.push(format!("for (uint {idx} = 0; {idx} < {count}; {idx}++) {{"));

                // Deserialize element into a temporary, then add to list.
                let elem_var = format!("_elem{depth}");
                let cs_inner_for_decl = self.cs_type_name(*inner_id);
                lines.push(format!("{cs_inner_for_decl} {elem_var} = default;"));
                self.emit_deserialize(lines, *inner_id, &elem_var, depth + 1);
                lines.push(format!("{target}.Add({elem_var});"));
                lines.push("} }".to_string());
            }
            RsTypeKind::WireOnly(WireOnly::Map(k_id, v_id)) => {
                let cs_k = self.cs_type_name(*k_id);
                let cs_v = self.cs_type_name(*v_id);
                let count = format!("_count{depth}");
                let idx = format!("_i{depth}");
                let k_var = format!("_key{depth}");
                let v_var = format!("_val{depth}");
                lines.push(format!(
                    "{{ var {count} = reader.ReadUInt32(); {target} = new Dictionary<{cs_k}, {cs_v}>((int){count});"
                ));
                lines.push(format!("for (uint {idx} = 0; {idx} < {count}; {idx}++) {{"));
                lines.push(format!("{cs_k} {k_var} = default;"));
                self.emit_deserialize(lines, *k_id, &k_var, depth + 1);
                lines.push(format!("{cs_v} {v_var} = default;"));
                self.emit_deserialize(lines, *v_id, &v_var, depth + 1);
                lines.push(format!("{target}[{k_var}] = {v_var};"));
                lines.push("} }".to_string());
            }
            RsTypeKind::Struct(s) => {
                let struct_name = self.cs_type_name(ty_id);
                lines.push(format!("{target} = new {struct_name}();"));
                for f in &s.fields {
                    self.emit_deserialize(lines, f.ty, &format!("{target}.{}", f.name), depth);
                }
            }
            _ => {
                lines.push(format!("/* unsupported wire type for {target} */"));
            }
        }
    }

    /// Emits C# statements that add the wire size of `val` to `_size`.
    fn emit_size(&self, lines: &mut Vec<String>, ty_id: interoptopus::inventory::TypeId, val: &str, depth: usize) {
        let Some(ty) = self.rs_types.get(&ty_id) else { return };
        match &ty.kind {
            RsTypeKind::Primitive(p) => {
                lines.push(format!("_size += {};", cs_primitive_size(*p)));
            }
            RsTypeKind::WireOnly(WireOnly::String) => {
                lines.push(format!("_size += 4 + System.Text.Encoding.UTF8.GetByteCount({val} ?? \"\");"));
            }
            RsTypeKind::WireOnly(WireOnly::Vec(inner_id)) => {
                let iter = format!("_item{depth}");
                lines.push(format!("_size += 4;"));
                lines.push(format!("if ({val} != null) {{ foreach (var {iter} in {val}) {{"));
                self.emit_size(lines, *inner_id, &iter, depth + 1);
                lines.push("} }".to_string());
            }
            RsTypeKind::WireOnly(WireOnly::Map(k_id, v_id)) => {
                let kv = format!("_kv{depth}");
                lines.push(format!("_size += 4;"));
                lines.push(format!("if ({val} != null) {{ foreach (var {kv} in {val}) {{"));
                self.emit_size(lines, *k_id, &format!("{kv}.Key"), depth + 1);
                self.emit_size(lines, *v_id, &format!("{kv}.Value"), depth + 1);
                lines.push("} }".to_string());
            }
            RsTypeKind::Struct(s) => {
                for f in &s.fields {
                    self.emit_size(lines, f.ty, &format!("{val}.{}", f.name), depth);
                }
            }
            _ => {}
        }
    }
}

fn cs_primitive_name(p: Primitive) -> &'static str {
    match p {
        Primitive::Void => "void",
        Primitive::Bool => "bool",
        Primitive::U8 => "byte",
        Primitive::U16 => "ushort",
        Primitive::U32 => "uint",
        Primitive::U64 => "ulong",
        Primitive::Usize => "nuint",
        Primitive::I8 => "sbyte",
        Primitive::I16 => "short",
        Primitive::I32 => "int",
        Primitive::I64 => "long",
        Primitive::Isize => "nint",
        Primitive::F32 => "float",
        Primitive::F64 => "double",
    }
}

fn cs_read_primitive(p: Primitive) -> &'static str {
    match p {
        Primitive::Bool => "reader.ReadByte() != 0",
        Primitive::U8 => "reader.ReadByte()",
        Primitive::U16 => "reader.ReadUInt16()",
        Primitive::U32 => "reader.ReadUInt32()",
        Primitive::U64 => "reader.ReadUInt64()",
        Primitive::I8 => "reader.ReadSByte()",
        Primitive::I16 => "reader.ReadInt16()",
        Primitive::I32 => "reader.ReadInt32()",
        Primitive::I64 => "reader.ReadInt64()",
        Primitive::F32 => "reader.ReadSingle()",
        Primitive::F64 => "reader.ReadDouble()",
        Primitive::Usize | Primitive::Isize => "reader.ReadInt64()", // platform-width
        Primitive::Void => "default",
    }
}

fn cs_primitive_size(p: Primitive) -> &'static str {
    match p {
        Primitive::Void => "0",
        Primitive::Bool | Primitive::U8 | Primitive::I8 => "1",
        Primitive::U16 | Primitive::I16 => "2",
        Primitive::U32 | Primitive::I32 | Primitive::F32 => "4",
        Primitive::U64 | Primitive::I64 | Primitive::F64 | Primitive::Usize | Primitive::Isize => "8",
    }
}
