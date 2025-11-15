//! ...

use crate::lang::types;
use crate::lang::types::TypeKind;
use crate::model::TypeId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, model_id_maps, model_type_kinds};
use interoptopus::lang;
use interoptopus::lang::types::Primitive;

#[derive(Default)]
pub struct Config {}

pub struct Pass {}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self {}
    }

    pub fn process(&mut self, id_map: &mut model_id_maps::Pass, kinds: &mut model_type_kinds::Pass, rs_types: &interoptopus::inventory::Types) -> ModelResult {
        for (rust_id, ty) in rs_types {
            let primitive = match ty.kind {
                lang::types::TypeKind::Primitive(x) => map(x),
                _ => continue,
            };

            let cs_id = TypeId::from_id(rust_id.id());
            id_map.set_rust_to_cs(*rust_id, cs_id);
            kinds.set_kind(cs_id, TypeKind::Primitive(primitive));
        }

        Ok(Unchanged)
    }
}

fn map(p: Primitive) -> types::Primitive {
    match p {
        Primitive::Void => types::Primitive::Void,
        Primitive::Bool => types::Primitive::Bool,
        Primitive::U8 => types::Primitive::Byte,
        Primitive::U16 => types::Primitive::UShort,
        Primitive::U32 => types::Primitive::UInt,
        Primitive::U64 => types::Primitive::ULong,
        Primitive::Usize => types::Primitive::NUInt,
        Primitive::I8 => types::Primitive::SByte,
        Primitive::I16 => types::Primitive::Short,
        Primitive::I32 => types::Primitive::Int,
        Primitive::I64 => types::Primitive::Long,
        Primitive::Isize => types::Primitive::NInt,
        Primitive::F32 => types::Primitive::Float,
        Primitive::F64 => types::Primitive::Double,
    }
}
