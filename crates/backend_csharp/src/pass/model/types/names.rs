//! Builds a map of C# `TypeId` to proper C# type names.

use crate::lang::TypeId;
use crate::lang::types::kind::{Primitive, TypeKind, TypePattern};
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo, model};
use crate::try_resolve;
use interoptopus_backends::casing::{rust_to_pascal, sanitize_delegate_name, sanitize_rust_name};
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

/// Like `resolve_name!`, but for building composed type names (Result, Slice, etc.).
/// For pointer types, resolves through the target to produce a unique name
/// (e.g., the target's name instead of `IntPtr`), preventing name collisions
/// when different pointer types are used inside generic wrappers.
macro_rules! resolve_compositional_name {
    ($self:expr, $id:expr, $kinds:expr, $pass_meta:expr) => {{
        if let Some(TypeKind::Pointer(p)) = $kinds.get($id) {
            match $self.names.get(&p.target) {
                Some(n) => n.as_str(),
                None => {
                    $pass_meta.lost_found.missing($self.info, crate::pass::MissingItem::CsType(p.target));
                    continue;
                }
            }
        } else {
            resolve_name!($self, $id, $pass_meta)
        }
    }};
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, names: HashMap::default() }
    }

    pub fn process(
        &mut self,
        pass_meta: &mut crate::pass::PassMeta,
        id_map: &model::id_map::Pass,
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
                TypeKind::Primitive(p) => primitive_name(*p).to_string(),
                TypeKind::Pointer(_) => "IntPtr".to_string(),
                TypeKind::Array(a) => {
                    let element = resolve_name!(self, a.ty, pass_meta);
                    format!("{element}[]")
                }
                TypeKind::TypePattern(p) => match p {
                    TypePattern::Bool => "Bool".to_string(),
                    TypePattern::CChar => "byte".to_string(),
                    TypePattern::CVoid => "void".to_string(),
                    TypePattern::CStrPointer => "string".to_string(),
                    TypePattern::Utf8String => "Utf8String".to_string(),
                    TypePattern::ApiVersion => "ulong".to_string(),
                    TypePattern::Slice(t) => format!("Slice{}", rust_to_pascal(resolve_compositional_name!(self, *t, kinds, pass_meta))),
                    TypePattern::SliceMut(t) => format!("SliceMut{}", rust_to_pascal(resolve_compositional_name!(self, *t, kinds, pass_meta))),
                    TypePattern::Vec(t) => format!("Vec{}", rust_to_pascal(resolve_compositional_name!(self, *t, kinds, pass_meta))),
                    TypePattern::Option(t, _) => format!("Option{}", rust_to_pascal(resolve_compositional_name!(self, *t, kinds, pass_meta))),
                    TypePattern::AsyncCallback(t) => "AsyncCallbackCommonNative".to_string(),
                    TypePattern::Wire(t) => {
                        // The inner type of Wire may not have a C# TypeKind (its fields use
                        // WireOnly types), so resolve the name from the Rust inventory directly.
                        let rust_name = rs_types.iter()
                            .find(|(rid, _)| id_map.ty(**rid) == Some(*t))
                            .map(|(_, ty)| sanitize_rust_name(&ty.name))
                            .unwrap_or_else(|| "Unknown".to_string());
                        format!("WireOf{}", rust_to_pascal(&rust_name))
                    }
                    TypePattern::Result(ok, err, _) => {
                        let ok_name = rust_to_pascal(resolve_compositional_name!(self, *ok, kinds, pass_meta));
                        let err_name = rust_to_pascal(resolve_compositional_name!(self, *err, kinds, pass_meta));
                        format!("Result{ok_name}{err_name}")
                    }
                },
                TypeKind::Delegate(_) => match &ty.kind {
                    // Bare fn pointers have signature-based names like "extern C fn(u8) -> u8"
                    interoptopus::lang::types::TypeKind::FnPointer(_) => sanitize_delegate_name(&ty.name),
                    // Named callbacks already have clean Rust names
                    _ => sanitize_rust_name(&ty.name),
                },
                _ => sanitize_rust_name(&ty.name),
            };

            self.names.insert(cs_id, cs_name);
            outcome.changed();
        }

        Ok(outcome)
    }

    pub fn set(&mut self, ty: TypeId, name: String) {
        self.names.insert(ty, name);
    }

    #[must_use]
    pub fn get(&self, ty: TypeId) -> Option<&String> {
        self.names.get(&ty)
    }
}

fn primitive_name(p: Primitive) -> &'static str {
    match p {
        Primitive::Void => "void",
        Primitive::Bool => "Bool",
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
