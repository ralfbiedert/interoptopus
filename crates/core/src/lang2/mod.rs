use crate::inventory2::Inventory;
use crate::lang2::constant::ConstantId;
use crate::lang2::function::FunctionId;
use crate::lang2::types::TypeId;

pub mod constant;
pub mod function;
pub mod meta;
pub mod types;

pub trait TypeInfo {
    fn id() -> TypeId;
}

pub trait FunctionInfo {
    fn id() -> FunctionId;
}

pub trait ConstantInfo {
    fn id() -> ConstantId;
}

pub trait Register {
    fn register(inventory: &mut Inventory);
}
