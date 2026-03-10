use crate::inventory::{FunctionId, Inventory, ServiceId, TypeId};

pub trait ServiceInfo {
    fn id() -> ServiceId;
    fn service() -> Service;
    fn register(inventory: &mut impl Inventory);
}

/// Combines a receiver, constructor, destructor and multiple methods in one entity.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Service {
    pub ty: TypeId,
    pub ctors: Vec<FunctionId>,
    pub destructor: FunctionId,
    pub methods: Vec<FunctionId>,
}

impl Service {
    pub fn new(ty: TypeId, ctors: Vec<FunctionId>, destructor: FunctionId, methods: Vec<FunctionId>) -> Self {
        Self { ty, ctors, destructor, methods }
    }
}
