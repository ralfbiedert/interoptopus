//! Maps Rust pointers (ReadPointer, ReadWritePointer) to C# pointers.

use crate::lang::types::{Pointer, TypeKind};
use crate::model::TypeId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};
use interoptopus::lang;

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
        id_map: &mut model::id::Pass,
        kinds: &mut model::types::kind::Pass,
        rs_types: &interoptopus::inventory::Types,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        for (rust_id, ty) in rs_types {
            skip_mapped!(id_map, rust_id);

            let rust_pointee_id = match &ty.kind {
                lang::types::TypeKind::ReadPointer(pointee) => pointee,
                lang::types::TypeKind::ReadWritePointer(pointee) => pointee,
                _ => continue,
            };

            // Create C# TypeId for the pointer
            let cs_id = TypeId::from_id(rust_id.id());

            // Try to convert the pointee type
            let cs_pointee_id = try_resolve!(id_map.ty(*rust_pointee_id), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_pointee_id));

            // Register the pointer type
            id_map.set_ty(*rust_id, cs_id);
            kinds.set_kind(cs_id, TypeKind::Pointer(Pointer::IntPtr(cs_pointee_id)));
            outcome.changed();
        }

        Ok(outcome)
    }
}
