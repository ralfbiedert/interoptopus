mod foreign;
mod id;
mod macros;
mod rust;

use crate::lang::constant::Constant;
use crate::lang::function::Function;
use crate::lang::service::Service;
use crate::lang::types::Type;
use std::collections::HashMap;

pub use foreign::ForeignInventory;
pub use id::{ConstantId, FunctionId, Id, ServiceId, TypeId, hash_str};
pub use rust::RustInventory;

pub type Types = HashMap<TypeId, Type>;
pub type Functions = HashMap<FunctionId, Function>;
pub type Constants = HashMap<ConstantId, Constant>;
pub type Services = HashMap<ServiceId, Service>;

pub trait Inventory {
    fn register_type(&mut self, id: TypeId, ty: Type);
    fn register_function(&mut self, id: FunctionId, function: Function);
    fn register_constant(&mut self, id: ConstantId, constant: Constant);
    fn register_service(&mut self, id: ServiceId, service: Service);
    fn register(&mut self, f: impl Fn(&mut Self)) -> &mut Self;
}
