//! ...

use crate::lang;
use crate::lang::types;
use crate::model::{TypeId, Types};
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, OutputResult};
use interoptopus::lang::types::{Primitive, TypeKind, TypePattern, VariantKind, WireOnly};
use std::collections::HashMap;

type RsToCs = HashMap<interoptopus::inventory::TypeId, TypeId>;

#[derive(Copy)]
struct TypeInfo<'a> {
    rs_types: &'a interoptopus::inventory::Types,
    cs_types: &'a mut Types,
    rs_to_cs: &'a mut RsToCs,
}

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    types: Types,
    rust_to_cs: RsToCs,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { types: Default::default(), rust_to_cs: Default::default() }
    }

    pub fn process(&mut self, rs_types: &interoptopus::inventory::Types) -> ModelResult {
        let info = TypeInfo { rs_types, cs_types, rs_to_cs: &mut self.rust_to_cs };

        for ty in rs_types {
            map_type_recursive(*ty.0, info);
        }
        Ok(Unchanged)
    }
}

fn map_type_recursive(type_id: interoptopus::inventory::TypeId, info: TypeInfo) {
    let ty = &info.rs_types[&type_id];

    // Handle recursion first
    match &ty.kind {
        TypeKind::Array(x) => map_type_recursive(x.ty, info),
        TypeKind::Primitive(_) => {}
        TypeKind::Struct(x) => {
            for field in &x.fields {
                map_type_recursive(field.ty, info);
            }
        }
        TypeKind::Enum(x) => {
            for variant in &x.variants {
                match variant.kind {
                    VariantKind::Unit(_) => {}
                    VariantKind::Tuple(x) => map_type_recursive(x, info),
                }
            }
        }
        TypeKind::FnPointer(x) => {
            for arg in &x.arguments {
                map_type_recursive(arg.ty, info);
            }
        }
        TypeKind::ReadPointer(x) => map_type_recursive(*x, info),
        TypeKind::ReadWritePointer(x) => map_type_recursive(*x, info),
        TypeKind::WireOnly(x) => match x {
            WireOnly::String => {}
            WireOnly::Vec(x) => map_type_recursive(*x, info),
            WireOnly::Map(x, y) => {
                map_type_recursive(*x, info);
                map_type_recursive(*y, info);
            }
        },
        TypeKind::Service => {}
        TypeKind::Opaque => {}
        TypeKind::TypePattern(x) => match x {
            TypePattern::Slice(x) => map_type_recursive(*x, info),
            TypePattern::SliceMut(x) => map_type_recursive(*x, info),
            TypePattern::Option(x) => map_type_recursive(*x, info),
            TypePattern::Result(x, y) => {
                map_type_recursive(*x, info);
                map_type_recursive(*y, info);
            }
            TypePattern::NamedCallback(sig) => {
                for x in &sig.arguments {
                    map_type_recursive(x.ty, info);
                }
            }
            TypePattern::AsyncCallback(x) => map_type_recursive(*x, info),
            TypePattern::Vec(x) => map_type_recursive(*x, info),
            TypePattern::CStrPointer => {}
            TypePattern::Utf8String => {}
            TypePattern::APIVersion => {}
            TypePattern::Bool => {}
            TypePattern::CChar => {}
            TypePattern::CVoid => {}
        }, // Register this type
    }

    // Here we know all deps have been mapped
    map_type_leaf(type_id, info);
}

fn map_type_leaf(type_id: interoptopus::inventory::TypeId, info: TypeInfo) {
    let ty = &info.rs_types[&type_id];

    let kind = match ty.kind {
        TypeKind::Primitive(x) => match x {
            Primitive::Void => types::TypeKind::Primitive(types::Primitive::Void),
            Primitive::Bool => {}
            Primitive::U8 => {}
            Primitive::U16 => {}
            Primitive::U32 => {}
            Primitive::U64 => {}
            Primitive::Usize => {}
            Primitive::I8 => {}
            Primitive::I16 => {}
            Primitive::I32 => {}
            Primitive::I64 => {}
            Primitive::Isize => {}
            Primitive::F32 => {}
            Primitive::F64 => {}
        },
        TypeKind::Array(_) => {}
        TypeKind::Struct(_) => {}
        TypeKind::Enum(_) => {}
        TypeKind::FnPointer(_) => {}
        TypeKind::ReadPointer(_) => {}
        TypeKind::Service => {}
        TypeKind::ReadWritePointer(_) => {}
        TypeKind::Opaque => {}
        TypeKind::WireOnly(_) => {}
        TypeKind::TypePattern(_) => {}
    };
}
