//! Builds a map of C# TypeId to proper C# type names.

use crate::lang::types::{Array, Primitive, TypeKind, TypePattern};
use crate::model::TypeId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};
use crate::try_resolve;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    names: HashMap<TypeId, String>,
}

/// Look up an already-resolved C# name from the map, reporting a missing
/// dependency and `continue`-ing the enclosing loop when not yet available.
macro_rules! resolve_name {
    ($self:expr, $id:expr, $pass_meta:expr) => {
        match $self.names.get(&$id) {
            Some(n) => n.as_str(),
            None => {
                $pass_meta.lost_found.missing($self.info, crate::pass::MissingItem::CsType($id));
                continue;
            }
        }
    };
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, names: Default::default() }
    }

    pub fn process(
        &mut self,
        pass_meta: &mut crate::pass::PassMeta,
        id_map: &model::id::Pass,
        kinds: &model::types::kind::Pass,
        rs_types: &interoptopus::inventory::Types,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        for (rust_id, ty) in rs_types {
            let cs_id = try_resolve!(id_map.ty(*rust_id), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_id));

            // Skip if we've already mapped this name
            if self.names.contains_key(&cs_id) {
                continue;
            }

            let cs_kind = try_resolve!(kinds.get(cs_id), pass_meta, self.info, crate::pass::MissingItem::CsType(cs_id));

            let cs_name = match cs_kind {
                TypeKind::Primitive(p) => primitive_name(p).to_string(),
                TypeKind::Pointer(_) => "IntPtr".to_string(),
                TypeKind::Array(a) => {
                    let element = resolve_name!(self, a.ty, pass_meta);
                    format!("{element}[]")
                }
                TypeKind::TypePattern(p) => match p {
                    TypePattern::Bool => "bool".to_string(),
                    TypePattern::CChar => "byte".to_string(),
                    TypePattern::CVoid => "void".to_string(),
                    TypePattern::CStrPointer => "CStrPtr".to_string(),
                    TypePattern::Utf8String => "Utf8String".to_string(),
                    TypePattern::ApiVersion => "ApiVersion".to_string(),
                    TypePattern::Slice(t) => format!("Slice{}", pascal(resolve_name!(self, *t, pass_meta))),
                    TypePattern::SliceMut(t) => format!("SliceMut{}", pascal(resolve_name!(self, *t, pass_meta))),
                    TypePattern::Vec(t) => format!("Vec{}", pascal(resolve_name!(self, *t, pass_meta))),
                    TypePattern::Option(t) => format!("Option{}", pascal(resolve_name!(self, *t, pass_meta))),
                    TypePattern::AsyncCallback(t) => format!("AsyncCallback{}", pascal(resolve_name!(self, *t, pass_meta))),
                    TypePattern::Result(ok, err) => {
                        let ok_name = pascal(resolve_name!(self, *ok, pass_meta));
                        let err_name = pascal(resolve_name!(self, *err, pass_meta));
                        format!("Result{}{}", ok_name, err_name)
                    }
                    TypePattern::NamedCallback(_) => sanitize_name(&ty.name),
                },
                TypeKind::Delegate(_) => sanitize_delegate_name(&ty.name),
                _ => sanitize_name(&ty.name),
            };

            self.names.insert(cs_id, cs_name);
            outcome.changed();
        }

        Ok(outcome)
    }

    pub fn name(&self, ty: TypeId) -> Option<&String> {
        self.names.get(&ty)
    }
}

/// Convert a C# type name to PascalCase for use in composite names.
fn pascal(name: &str) -> String {
    let mut result = String::with_capacity(name.len());
    let mut capitalize_next = true;
    for c in name.chars() {
        if c == '_' || c == ' ' {
            capitalize_next = true;
        } else if capitalize_next {
            result.extend(c.to_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    result
}

/// Sanitize a Rust delegate/fn-pointer name into a valid C# identifier.
///
/// Strips `extern "C" fn` prefix and all non-alphanumeric characters,
/// then PascalCases the fragments. For example:
/// - `extern "C" fn(u8) -> u8` → `FnU8U8`
/// - `extern "C" fn(Vec3f32) -> ()` → `FnVec3f32`
fn sanitize_delegate_name(name: &str) -> String {
    // Strip extern "C" prefix, keep "fn" onwards
    let stripped = name
        .strip_prefix("extern \"C\" ")
        .or_else(|| name.strip_prefix("extern \"C\" "))
        .unwrap_or(name);

    // Strip return type `-> ()` (void) entirely
    let stripped = stripped.replace("-> ()", "");

    let mut result = String::with_capacity(stripped.len());
    let mut capitalize_next = true;

    for c in stripped.chars() {
        if c.is_alphanumeric() {
            if capitalize_next {
                result.extend(c.to_uppercase());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        } else {
            // Any non-alphanumeric char is a word boundary
            capitalize_next = true;
        }
    }

    result
}

/// Sanitize a Rust type name into a valid C# identifier.
///
/// Strips angle brackets, commas, semicolons, square brackets, and spaces,
/// then PascalCases the fragments. For example:
/// - `Weird2<u8, 5>` → `Weird2U85`
/// - `[u8; 5]` → `U85` (arrays should be handled before reaching this)
/// - `MyStruct` → `MyStruct` (unchanged)
fn sanitize_name(name: &str) -> String {
    let mut result = String::with_capacity(name.len());
    let mut capitalize_next = true;

    for c in name.chars() {
        match c {
            '<' | '>' | ',' | ';' | '[' | ']' | ' ' => {
                capitalize_next = true;
            }
            '_' => {
                capitalize_next = true;
            }
            _ if capitalize_next => {
                result.extend(c.to_uppercase());
                capitalize_next = false;
            }
            _ => {
                result.push(c);
            }
        }
    }

    result
}

fn primitive_name(p: &Primitive) -> &'static str {
    match p {
        Primitive::Void => "void",
        Primitive::Bool => "bool",
        Primitive::Byte => "byte",
        Primitive::UShort => "ushort",
        Primitive::UInt => "uint",
        Primitive::ULong => "ulong",
        Primitive::NUInt => "nuint",
        Primitive::SByte => "sbyte",
        Primitive::Short => "short",
        Primitive::Int => "int",
        Primitive::Long => "long",
        Primitive::NInt => "nint",
        Primitive::Float => "float",
        Primitive::Double => "double",
    }
}
