mod id;

use crate::lang2::types::{Type, TypeId};
pub use id::Id;
use std::collections::HashMap;

pub struct Inventory {
    types: HashMap<TypeId, Type>,
}

impl Inventory {
    pub fn register_type(&mut self, id: TypeId, type_: Type) {
        self.types.entry(id).or_insert(type_);
    }
}
