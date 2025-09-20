use crate::inventory::{FunctionId, Inventory, ServiceId, TypeId};

pub trait ServiceInfo {
    fn id() -> ServiceId;
    fn service() -> Service;
    fn register(inventory: &mut Inventory);
}

/// Combines a receiver, constructor, destructor and multiple methods in one entity.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Service {
    ty: TypeId,
    ctors: Vec<FunctionId>,
    destructor: FunctionId,
    methods: Vec<FunctionId>,
}

impl Service {
    pub fn new(ty: TypeId, ctors: Vec<FunctionId>, destructor: FunctionId, methods: Vec<FunctionId>) -> Self {
        Self {
            ty,
            ctors,
            destructor,
            methods,
        }
    }
}
