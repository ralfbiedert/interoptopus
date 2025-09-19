use crate::inventory::Inventory;

pub mod constant;
pub mod function;
pub mod meta;
pub mod service;
pub mod types;

pub trait Register {
    fn register(inventory: &mut Inventory);
}
