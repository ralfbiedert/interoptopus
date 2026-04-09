//! Wire output passes for `Wire<T>` C# code generation.
//!
//! Split into focused submodules:
//! - [`WireCodeGen`] — Shared C# code generation logic (type mapping, serialize/deserialize/size emission)
//! - [`wire_type`] — Renders `WireOf*` structs for each `Wire<T>` pattern
//! - [`helper_classes`] — Emits managed classes for nested structs with `WireOnly` fields
//! - [`all`] — Assembles `wire_type` and `helper_classes` results per output file

pub mod all;
pub mod helper_classes;
pub mod wire_type;

use interoptopus::inventory::{TypeId, Types as RsTypes};
use interoptopus::lang::types::{Array, Layout, Primitive, Struct, TypeKind as RsTypeKind, WireOnly};

/// Generates C# serialization code for the wire format by walking Rust types.
///
/// A shared utility used by the wire output passes. It walks the Rust type graph recursively
/// translating primitives, `WireOnly` types, and user structs into inline C# statements.
pub struct WireCodeGen<'a> {
    pub rs_types: &'a RsTypes,
}

impl WireCodeGen<'_> {
    /// Maps a Rust type to its C# managed type name.
    #[must_use]
    pub fn cs_type_name(&self, ty_id: TypeId) -> String {
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
            RsTypeKind::WireOnly(WireOnly::Option(inner)) => {
                let inner_name = self.cs_type_name(*inner);
                if is_cs_value_type(*inner, self.rs_types) {
                    format!("{inner_name}?")
                } else {
                    inner_name
                }
            }
            RsTypeKind::Struct(_) => ty.name.clone(),
            RsTypeKind::Enum(_) => ty.name.clone(),
            RsTypeKind::Array(arr) => format!("{}[]", self.cs_type_name(arr.ty)),
            RsTypeKind::TypePattern(interoptopus::lang::types::TypePattern::Option(inner)) => {
                let inner_name = self.cs_type_name(*inner);
                if is_cs_value_type(*inner, self.rs_types) {
                    format!("{inner_name}?")
                } else {
                    inner_name
                }
            }
            _ => "object".to_string(),
        }
    }

    /// Generates the serialize method body for a struct.
    #[must_use]
    pub fn serialize_struct_body(&self, s: &Struct, val: &str) -> String {
        let mut lines = Vec::new();
        for f in &s.fields {
            let access = format!("{val}.{}", f.name);
            self.emit_serialize(&mut lines, f.ty, &access, 0, 0);
        }
        lines.join("\n")
    }

    /// Generates the deserialize method body for a struct.
    #[must_use]
    pub fn deserialize_struct_body(&self, s: &Struct, type_name: &str) -> String {
        let mut lines = Vec::new();
        lines.push(format!("var result = new {type_name}();"));
        for f in &s.fields {
            let target = format!("result.{}", f.name);
            self.emit_deserialize(&mut lines, f.ty, &target, 0, 0);
        }
        lines.push("return result;".to_string());
        lines.join("\n")
    }

    /// Generates the size calculation body for a struct.
    #[must_use]
    pub fn size_struct_body(&self, s: &Struct, val: &str) -> String {
        let mut lines = Vec::new();
        lines.push("var _size = 0;".to_string());
        for f in &s.fields {
            let access = format!("{val}.{}", f.name);
            self.emit_size(&mut lines, f.ty, &access, 0, 0);
        }
        lines.push("return _size;".to_string());
        lines.join("\n")
    }

    /// Emits C# statements to serialize a value of the given Rust type.
    pub fn emit_serialize(&self, lines: &mut Vec<String>, ty_id: TypeId, val: &str, depth: usize, indent: usize) {
        let Some(ty) = self.rs_types.get(&ty_id) else { return };
        let p = pad(indent);
        match &ty.kind {
            RsTypeKind::Primitive(prim) => {
                if *prim == Primitive::Bool {
                    lines.push(format!("{p}writer.Write({val} ? (byte)1 : (byte)0);"));
                } else {
                    lines.push(format!("{p}writer.Write({val});"));
                }
            }
            RsTypeKind::WireOnly(WireOnly::String) => {
                lines.push(format!("{p}{{ var _bytes = System.Text.Encoding.UTF8.GetBytes({val} ?? \"\"); writer.Write((uint)_bytes.Length); writer.Write(_bytes); }}"));
            }
            RsTypeKind::WireOnly(WireOnly::Vec(inner_id)) => {
                let iter = format!("_item{depth}");
                let pi = pad(indent + 1);
                lines.push(format!("{p}writer.Write((uint)({val}?.Count ?? 0));"));
                lines.push(format!("{p}if ({val} != null)"));
                lines.push(format!("{p}{{"));
                lines.push(format!("{pi}foreach (var {iter} in {val})"));
                lines.push(format!("{pi}{{"));
                self.emit_serialize(lines, *inner_id, &iter, depth + 1, indent + 2);
                lines.push(format!("{pi}}}"));
                lines.push(format!("{p}}}"));
            }
            RsTypeKind::WireOnly(WireOnly::Map(k_id, v_id)) => {
                let kv = format!("_kv{depth}");
                let pi = pad(indent + 1);
                lines.push(format!("{p}writer.Write((uint)({val}?.Count ?? 0));"));
                lines.push(format!("{p}if ({val} != null)"));
                lines.push(format!("{p}{{"));
                lines.push(format!("{pi}foreach (var {kv} in {val})"));
                lines.push(format!("{pi}{{"));
                self.emit_serialize(lines, *k_id, &format!("{kv}.Key"), depth + 1, indent + 2);
                self.emit_serialize(lines, *v_id, &format!("{kv}.Value"), depth + 1, indent + 2);
                lines.push(format!("{pi}}}"));
                lines.push(format!("{p}}}"));
            }
            RsTypeKind::WireOnly(WireOnly::Option(inner_id)) => {
                self.emit_option_serialize(lines, *inner_id, val, depth, indent);
            }
            RsTypeKind::Array(arr) => {
                let idx = format!("_i{depth}");
                let pi = pad(indent + 1);
                lines.push(format!("{p}for (int {idx} = 0; {idx} < {len}; {idx}++)", len = arr.len));
                lines.push(format!("{p}{{"));
                self.emit_serialize(lines, arr.ty, &format!("{val}[{idx}]"), depth + 1, indent + 1);
                lines.push(format!("{p}}}"));
            }
            RsTypeKind::Enum(_) => {
                lines.push(format!("{p}writer.Write({val}.ToUnmanaged()._variant);"));
            }
            RsTypeKind::Struct(s) => {
                for f in &s.fields {
                    self.emit_serialize(lines, f.ty, &format!("{val}.{}", f.name), depth, indent);
                }
            }
            RsTypeKind::TypePattern(interoptopus::lang::types::TypePattern::Option(inner_id)) => {
                self.emit_option_serialize(lines, *inner_id, val, depth, indent);
            }
            _ => {
                lines.push(format!("{p}/* unsupported wire type for {val} */"));
            }
        }
    }

    /// Emits C# statements to deserialize a value and assign it to `target`.
    pub fn emit_deserialize(&self, lines: &mut Vec<String>, ty_id: TypeId, target: &str, depth: usize, indent: usize) {
        let Some(ty) = self.rs_types.get(&ty_id) else { return };
        let p = pad(indent);
        match &ty.kind {
            RsTypeKind::Primitive(prim) => {
                lines.push(format!("{p}{target} = {};", cs_read_primitive(*prim)));
            }
            RsTypeKind::WireOnly(WireOnly::String) => {
                lines.push(format!(
                    "{p}{{ var _len = reader.ReadUInt32(); {target} = _len > 0 ? System.Text.Encoding.UTF8.GetString(reader.ReadBytes((int)_len)) : \"\"; }}"
                ));
            }
            RsTypeKind::WireOnly(WireOnly::Vec(inner_id)) => {
                let cs_inner = self.cs_type_name(*inner_id);
                let count = format!("_count{depth}");
                let idx = format!("_i{depth}");
                let elem_var = format!("_elem{depth}");
                let pi = pad(indent + 1);
                lines.push(format!("{p}{{"));
                lines.push(format!("{pi}var {count} = reader.ReadUInt32();"));
                lines.push(format!("{pi}{target} = new List<{cs_inner}>((int){count});"));
                lines.push(format!("{pi}for (uint {idx} = 0; {idx} < {count}; {idx}++)"));
                lines.push(format!("{pi}{{"));
                lines.push(format!("{pi}    {cs_inner} {elem_var} = default;"));
                self.emit_deserialize(lines, *inner_id, &elem_var, depth + 1, indent + 2);
                lines.push(format!("{pi}    {target}.Add({elem_var});"));
                lines.push(format!("{pi}}}"));
                lines.push(format!("{p}}}"));
            }
            RsTypeKind::WireOnly(WireOnly::Map(k_id, v_id)) => {
                let cs_k = self.cs_type_name(*k_id);
                let cs_v = self.cs_type_name(*v_id);
                let count = format!("_count{depth}");
                let idx = format!("_i{depth}");
                let k_var = format!("_key{depth}");
                let v_var = format!("_val{depth}");
                let pi = pad(indent + 1);
                let pi2 = pad(indent + 2);
                lines.push(format!("{p}{{"));
                lines.push(format!("{pi}var {count} = reader.ReadUInt32();"));
                lines.push(format!("{pi}{target} = new Dictionary<{cs_k}, {cs_v}>((int){count});"));
                lines.push(format!("{pi}for (uint {idx} = 0; {idx} < {count}; {idx}++)"));
                lines.push(format!("{pi}{{"));
                lines.push(format!("{pi2}{cs_k} {k_var} = default;"));
                self.emit_deserialize(lines, *k_id, &k_var, depth + 1, indent + 2);
                lines.push(format!("{pi2}{cs_v} {v_var} = default;"));
                self.emit_deserialize(lines, *v_id, &v_var, depth + 1, indent + 2);
                lines.push(format!("{pi2}{target}[{k_var}] = {v_var};"));
                lines.push(format!("{pi}}}"));
                lines.push(format!("{p}}}"));
            }
            RsTypeKind::WireOnly(WireOnly::Option(inner_id)) => {
                self.emit_option_deserialize(lines, *inner_id, target, depth, indent);
            }
            RsTypeKind::Array(arr) => {
                let cs_elem = self.cs_type_name(arr.ty);
                let idx = format!("_i{depth}");
                let pi = pad(indent + 1);
                lines.push(format!("{p}{target} = new {cs_elem}[{len}];", len = arr.len));
                lines.push(format!("{p}for (int {idx} = 0; {idx} < {len}; {idx}++)", len = arr.len));
                lines.push(format!("{p}{{"));
                self.emit_deserialize(lines, arr.ty, &format!("{target}[{idx}]"), depth + 1, indent + 1);
                lines.push(format!("{p}}}"));
            }
            RsTypeKind::Enum(e) => {
                let enum_name = self.cs_type_name(ty_id);
                let read_expr = cs_read_primitive(enum_repr_primitive(e));
                lines.push(format!("{p}{{ var _u = new {enum_name}.Unmanaged(); _u._variant = {read_expr}; {target} = _u.ToManaged(); }}"));
            }
            RsTypeKind::Struct(s) => {
                let struct_name = self.cs_type_name(ty_id);
                lines.push(format!("{p}{target} = new {struct_name}();"));
                for f in &s.fields {
                    self.emit_deserialize(lines, f.ty, &format!("{target}.{}", f.name), depth, indent);
                }
            }
            RsTypeKind::TypePattern(interoptopus::lang::types::TypePattern::Option(inner_id)) => {
                self.emit_option_deserialize(lines, *inner_id, target, depth, indent);
            }
            _ => {
                lines.push(format!("{p}/* unsupported wire type for {target} */"));
            }
        }
    }

    /// Emits C# statements that add the wire size of `val` to `_size`.
    pub fn emit_size(&self, lines: &mut Vec<String>, ty_id: TypeId, val: &str, depth: usize, indent: usize) {
        let Some(ty) = self.rs_types.get(&ty_id) else { return };
        let p = pad(indent);
        match &ty.kind {
            RsTypeKind::Primitive(prim) => {
                lines.push(format!("{p}_size += {};", cs_primitive_size(*prim)));
            }
            RsTypeKind::WireOnly(WireOnly::String) => {
                lines.push(format!("{p}_size += 4 + System.Text.Encoding.UTF8.GetByteCount({val} ?? \"\");"));
            }
            RsTypeKind::WireOnly(WireOnly::Vec(inner_id)) => {
                let iter = format!("_item{depth}");
                let pi = pad(indent + 1);
                lines.push(format!("{p}_size += 4;"));
                lines.push(format!("{p}if ({val} != null)"));
                lines.push(format!("{p}{{"));
                lines.push(format!("{pi}foreach (var {iter} in {val})"));
                lines.push(format!("{pi}{{"));
                self.emit_size(lines, *inner_id, &iter, depth + 1, indent + 2);
                lines.push(format!("{pi}}}"));
                lines.push(format!("{p}}}"));
            }
            RsTypeKind::WireOnly(WireOnly::Map(k_id, v_id)) => {
                let kv = format!("_kv{depth}");
                let pi = pad(indent + 1);
                lines.push(format!("{p}_size += 4;"));
                lines.push(format!("{p}if ({val} != null)"));
                lines.push(format!("{p}{{"));
                lines.push(format!("{pi}foreach (var {kv} in {val})"));
                lines.push(format!("{pi}{{"));
                self.emit_size(lines, *k_id, &format!("{kv}.Key"), depth + 1, indent + 2);
                self.emit_size(lines, *v_id, &format!("{kv}.Value"), depth + 1, indent + 2);
                lines.push(format!("{pi}}}"));
                lines.push(format!("{p}}}"));
            }
            RsTypeKind::WireOnly(WireOnly::Option(inner_id)) => {
                self.emit_option_size(lines, *inner_id, val, depth, indent);
            }
            RsTypeKind::Array(arr) => {
                let idx = format!("_i{depth}");
                let pi = pad(indent + 1);
                lines.push(format!("{p}for (int {idx} = 0; {idx} < {len}; {idx}++)", len = arr.len));
                lines.push(format!("{p}{{"));
                self.emit_size(lines, arr.ty, &format!("{val}[{idx}]"), depth + 1, indent + 1);
                lines.push(format!("{p}}}"));
            }
            RsTypeKind::Enum(e) => {
                let prim = enum_repr_primitive(e);
                lines.push(format!("{p}_size += {};", cs_primitive_size(prim)));
            }
            RsTypeKind::Struct(s) => {
                for f in &s.fields {
                    self.emit_size(lines, f.ty, &format!("{val}.{}", f.name), depth, indent);
                }
            }
            RsTypeKind::TypePattern(interoptopus::lang::types::TypePattern::Option(inner_id)) => {
                self.emit_option_size(lines, *inner_id, val, depth, indent);
            }
            _ => {}
        }
    }

    fn emit_option_serialize(&self, lines: &mut Vec<String>, inner_id: TypeId, val: &str, depth: usize, indent: usize) {
        let p = pad(indent);
        if is_cs_value_type(inner_id, self.rs_types) {
            lines.push(format!("{p}writer.Write((byte)({val}.HasValue ? 1 : 0));"));
            lines.push(format!("{p}if ({val}.HasValue)"));
            lines.push(format!("{p}{{"));
            self.emit_serialize(lines, inner_id, &format!("{val}.Value"), depth, indent + 1);
        } else {
            lines.push(format!("{p}writer.Write((byte)({val} != null ? 1 : 0));"));
            lines.push(format!("{p}if ({val} != null)"));
            lines.push(format!("{p}{{"));
            self.emit_serialize(lines, inner_id, val, depth, indent + 1);
        }
        lines.push(format!("{p}}}"));
    }

    fn emit_option_deserialize(&self, lines: &mut Vec<String>, inner_id: TypeId, target: &str, depth: usize, indent: usize) {
        let p = pad(indent);
        let pi = pad(indent + 1);
        let pi2 = pad(indent + 2);
        let has_var = format!("_has{depth}");
        lines.push(format!("{p}{{"));
        lines.push(format!("{pi}var {has_var} = reader.ReadByte() != 0;"));
        lines.push(format!("{pi}if ({has_var})"));
        lines.push(format!("{pi}{{"));
        if is_cs_value_type(inner_id, self.rs_types) {
            let cs_inner = self.cs_type_name(inner_id);
            let tmp_var = format!("_optVal{depth}");
            lines.push(format!("{pi2}{cs_inner} {tmp_var} = default;"));
            self.emit_deserialize(lines, inner_id, &tmp_var, depth + 1, indent + 2);
            lines.push(format!("{pi2}{target} = {tmp_var};"));
        } else {
            self.emit_deserialize(lines, inner_id, target, depth + 1, indent + 2);
        }
        lines.push(format!("{pi}}}"));
        lines.push(format!("{pi}else"));
        lines.push(format!("{pi}{{"));
        lines.push(format!("{pi2}{target} = null;"));
        lines.push(format!("{pi}}}"));
        lines.push(format!("{p}}}"));
    }

    fn emit_option_size(&self, lines: &mut Vec<String>, inner_id: TypeId, val: &str, depth: usize, indent: usize) {
        let p = pad(indent);
        lines.push(format!("{p}_size += 1;"));
        if is_cs_value_type(inner_id, self.rs_types) {
            lines.push(format!("{p}if ({val}.HasValue)"));
            lines.push(format!("{p}{{"));
            self.emit_size(lines, inner_id, &format!("{val}.Value"), depth, indent + 1);
        } else {
            lines.push(format!("{p}if ({val} != null)"));
            lines.push(format!("{p}{{"));
            self.emit_size(lines, inner_id, val, depth, indent + 1);
        }
        lines.push(format!("{p}}}"));
    }
}

fn enum_repr_primitive(e: &interoptopus::lang::types::Enum) -> Primitive {
    match e.repr.layout {
        Layout::Primitive(p) => p,
        _ => Primitive::U32,
    }
}

fn pad(indent: usize) -> String {
    "    ".repeat(indent)
}

fn cs_primitive_name(p: Primitive) -> &'static str {
    match p {
        Primitive::Void => "void",
        Primitive::Bool => "bool",
        Primitive::U8 => "byte",
        Primitive::U16 => "ushort",
        Primitive::U32 => "uint",
        Primitive::U64 => "ulong",
        Primitive::I8 => "sbyte",
        Primitive::I16 => "short",
        Primitive::I32 => "int",
        Primitive::I64 => "long",
        Primitive::F32 => "float",
        Primitive::F64 => "double",
        Primitive::Usize | Primitive::Isize => "long",
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
        Primitive::Usize | Primitive::Isize => "reader.ReadInt64()",
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

/// Returns `true` if the Rust type maps to a C# value type (struct/primitive/enum)
/// rather than a reference type (class, string, List, Dictionary).
/// Structs with `WireOnly` fields are emitted as C# classes, so they are reference types.
fn is_cs_value_type(ty_id: TypeId, rs_types: &RsTypes) -> bool {
    let Some(ty) = rs_types.get(&ty_id) else { return false };
    match &ty.kind {
        RsTypeKind::Primitive(_) | RsTypeKind::Enum(_) | RsTypeKind::Array(_) => true,
        RsTypeKind::Struct(s) => !s.fields.iter().any(|f| contains_wireonly(f.ty, rs_types, &mut std::collections::HashSet::new())),
        _ => false,
    }
}

fn contains_wireonly(ty_id: TypeId, rs_types: &RsTypes, visited: &mut std::collections::HashSet<TypeId>) -> bool {
    if !visited.insert(ty_id) {
        return false;
    }
    let Some(ty) = rs_types.get(&ty_id) else { return false };
    match &ty.kind {
        RsTypeKind::WireOnly(_) => true,
        RsTypeKind::Struct(s) => s.fields.iter().any(|f| contains_wireonly(f.ty, rs_types, visited)),
        RsTypeKind::TypePattern(interoptopus::lang::types::TypePattern::Option(inner)) => contains_wireonly(*inner, rs_types, visited),
        RsTypeKind::TypePattern(interoptopus::lang::types::TypePattern::Result(ok, err)) => {
            contains_wireonly(*ok, rs_types, visited) || contains_wireonly(*err, rs_types, visited)
        }
        _ => false,
    }
}
