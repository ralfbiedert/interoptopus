use crate::inventory::{ConstantId, FunctionId, Inventory, ServiceId, TypeId};
use crate::lang::constant::Constant;
use crate::lang::function::Function;
use crate::lang::service::Service;
use crate::lang::types::Type;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::mem::swap;

#[derive(Default)]
pub struct ForeignInventory {
    pub types: HashMap<TypeId, Type>,
    _guard: PhantomData<()>,
}

impl ForeignInventory {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_type(&mut self, id: TypeId, ty: Type) {
        self.types.entry(id).or_insert(ty);
    }

    #[must_use]
    pub fn register(&mut self, f: impl Fn(&mut Self)) -> &mut Self {
        f(self);
        self
    }

    #[must_use]
    pub fn validate(&mut self) -> Self {
        let mut rval = Self::new();
        swap(&mut rval, self);
        rval
    }
}

impl Inventory for ForeignInventory {
    fn register_type(&mut self, id: TypeId, ty: Type) {
        self.types.entry(id).or_insert(ty);
    }

    fn register_function(&mut self, _: FunctionId, _: Function) {
        panic!("Invalid operation")
    }

    fn register_constant(&mut self, _: ConstantId, _: Constant) {
        todo!()
    }

    fn register_service(&mut self, _: ServiceId, _: Service) {
        panic!("Invalid operation")
    }

    fn register(&mut self, f: impl Fn(&mut Self)) -> &mut Self {
        f(self);
        self
    }
}
