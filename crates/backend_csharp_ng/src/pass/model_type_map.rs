//! ...

use crate::model::Types;
use crate::pass::ProcessError;
use interoptopus::lang::types::{TypeKind, TypePattern, VariantKind, WireOnly};

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    types: Types,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { types: Default::default() }
    }

    pub fn process(&mut self, rust_types: &interoptopus::inventory::Types) -> ProcessError {
        for ty in rust_types {
            map_type_recursive(*ty.0, &rust_types, &mut self.types);
        }
        Ok(())
    }
}

fn map_type_recursive(type_id: interoptopus::inventory::TypeId, rs_types: &interoptopus::inventory::Types, cs_types: &mut Types) {
    let ty = &rs_types[&type_id];

    // Handle recursion first
    match &ty.kind {
        TypeKind::Array(x) => map_type_recursive(x.ty, rs_types, cs_types),
        TypeKind::Primitive(_) => {}
        TypeKind::Struct(x) => {
            for field in &x.fields {
                map_type_recursive(field.ty, rs_types, cs_types);
            }
        }
        TypeKind::Enum(x) => {
            for variant in &x.variants {
                match variant.kind {
                    VariantKind::Unit(_) => {}
                    VariantKind::Tuple(x) => map_type_recursive(x, rs_types, cs_types),
                }
            }
        }
        TypeKind::FnPointer(x) => {
            for arg in &x.arguments {
                map_type_recursive(arg.ty, rs_types, cs_types);
            }
        }
        TypeKind::ReadPointer(x) => map_type_recursive(*x, rs_types, cs_types),
        TypeKind::ReadWritePointer(x) => map_type_recursive(*x, rs_types, cs_types),
        TypeKind::WireOnly(x) => match x {
            WireOnly::String => {}
            WireOnly::Vec(x) => map_type_recursive(*x, rs_types, cs_types),
            WireOnly::Map(x, y) => {
                map_type_recursive(*x, rs_types, cs_types);
                map_type_recursive(*y, rs_types, cs_types);
            }
        },
        TypeKind::Service => {}
        TypeKind::Opaque => {}
        TypeKind::TypePattern(x) => match x {
            TypePattern::Slice(x) => map_type_recursive(*x, rs_types, cs_types),
            TypePattern::SliceMut(x) => map_type_recursive(*x, rs_types, cs_types),
            TypePattern::Option(x) => map_type_recursive(*x, rs_types, cs_types),
            TypePattern::Result(x, y) => {
                map_type_recursive(*x, rs_types, cs_types);
                map_type_recursive(*y, rs_types, cs_types);
            }
            TypePattern::NamedCallback(sig) => {
                for x in &sig.arguments {
                    map_type_recursive(x.ty, rs_types, cs_types);
                }
            }
            TypePattern::AsyncCallback(x) => map_type_recursive(*x, rs_types, cs_types),
            TypePattern::Vec(x) => map_type_recursive(*x, rs_types, cs_types),
            TypePattern::CStrPointer => {}
            TypePattern::Utf8String => {}
            TypePattern::APIVersion => {}
            TypePattern::Bool => {}
            TypePattern::CChar => {}
            TypePattern::CVoid => {}
        }, // Register this type
    }

    // Here we know all deps have been mapped
    map_type_leaf(type_id, rs_types, cs_types);
}

fn map_type_leaf(type_id: interoptopus::inventory::TypeId, rs_types: &interoptopus::inventory::Types, cs_types: &mut Types) {
    // let ty = Type {
    //     namespace: (),
    //     name: "".to_string(),
    //     kind: TypeKind::Service,
    // }
}
