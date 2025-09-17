mod id;

use crate::lang2::constant::{Constant, ConstantId};
use crate::lang2::function::{Function, FunctionId};
use crate::lang2::types::{Type, TypeId};
pub use id::Id;
use std::collections::HashMap;

pub struct Inventory {
    types: HashMap<TypeId, Type>,
    functions: HashMap<FunctionId, Function>,
    constants: HashMap<ConstantId, Constant>,
}

impl Inventory {
    pub fn register_type(&mut self, id: TypeId, type_: Type) {
        self.types.entry(id).or_insert(type_);
    }

    pub fn register_function(&mut self, id: FunctionId, function: Function) {
        self.functions.entry(id).or_insert(function);
    }

    pub fn register_constant(&mut self, id: ConstantId, constant: Constant) {
        self.constants.entry(id).or_insert(constant);
    }
}
