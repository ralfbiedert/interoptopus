use crate::inventory::Symbol::Type;
use crate::inventory2::Inventory;
use crate::lang2::info::{Register, TypeInfo};
use crate::lang2::types::{Type, TypeId};

pub enum Primitive {
    U8,
    U16,
    U32,
    U64,
    Usize,
    I8,
    I16,
    I32,
    I64,
}

impl TypeInfo for u32 {
    fn id() -> TypeId {
        TypeId::new(123)
    }
}

impl Register for u32 {
    fn register(inventory: &mut Inventory) {
        let type_id = Self::id();
        let type_ = Type {};
        _ = inventory.register_type(type_id)
    }
}
