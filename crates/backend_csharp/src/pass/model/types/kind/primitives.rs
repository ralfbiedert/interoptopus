//! ...

use crate::lang::types::kind;
use crate::lang::types::kind::TypeKind;
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo, model};
use crate::{skip_mapped, try_extract_kind, try_resolve};
use interoptopus::lang::types::Primitive;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() } }
    }

    pub fn process(
        &mut self,
        pass_meta: &mut crate::pass::PassMeta,
        id_map: &model::id_map::Pass,
        kinds: &mut model::types::kind::Pass,
        rs_types: &interoptopus::inventory::Types,
    ) -> ModelResult {
        for (rust_id, ty) in rs_types {
            skip_mapped!(kinds, id_map, rust_id);
            let primitive = try_extract_kind!(ty, Primitive);
            let primitive = map(*primitive);
            let cs_id = try_resolve!(id_map.ty(*rust_id), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_id));
            kinds.set(cs_id, TypeKind::Primitive(primitive));
        }

        Ok(Unchanged)
    }
}

fn map(p: Primitive) -> kind::Primitive {
    match p {
        Primitive::Void => kind::Primitive::Void,
        Primitive::Bool => kind::Primitive::Bool,
        Primitive::U8 => kind::Primitive::Byte,
        Primitive::U16 => kind::Primitive::UShort,
        Primitive::U32 => kind::Primitive::UInt,
        Primitive::U64 => kind::Primitive::ULong,
        Primitive::Usize => kind::Primitive::NUInt,
        Primitive::I8 => kind::Primitive::SByte,
        Primitive::I16 => kind::Primitive::Short,
        Primitive::I32 => kind::Primitive::Int,
        Primitive::I64 => kind::Primitive::Long,
        Primitive::Isize => kind::Primitive::NInt,
        Primitive::F32 => kind::Primitive::Float,
        Primitive::F64 => kind::Primitive::Double,
    }
}
