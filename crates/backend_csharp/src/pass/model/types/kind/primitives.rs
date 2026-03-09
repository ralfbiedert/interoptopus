//! ...

use crate::lang::types;
use crate::lang::types::TypeKind;
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};
use crate::{skip_mapped, try_extract_kind, try_resolve};
use interoptopus::lang::types::Primitive;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() } }
    }

    pub fn process(
        &mut self,
        pass_meta: &mut crate::pass::PassMeta,
        id_map: &model::id::Pass,
        kinds: &mut model::types::kind::Pass,
        rs_types: &interoptopus::inventory::Types,
    ) -> ModelResult {
        for (rust_id, ty) in rs_types {
            skip_mapped!(kinds, id_map, rust_id);
            let primitive = try_extract_kind!(ty, Primitive);
            let primitive = map(*primitive);
            let cs_id = try_resolve!(id_map.ty(*rust_id), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_id));
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
