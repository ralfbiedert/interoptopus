use crate::inventory2::Inventory;
use crate::lang2::types::TypeId;

pub trait TypeInfo {
    fn id() -> TypeId;
}

pub trait Register {
    fn register(inventory: &mut Inventory);
}
