use crate::inventory::{Constants, ConstantId, Functions, FunctionId, Inventory, Services, ServiceId, Types, TypeId};
use crate::lang::constant::Constant;
use crate::lang::function::Function;
use crate::lang::service::Service;
use crate::lang::types::Type;
use std::marker::PhantomData;
use std::mem::swap;

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ForeignInventory {
    pub types: Types,
    pub functions: Functions,
    pub constants: Constants,
    pub services: Services,
    #[cfg_attr(feature = "serde", serde(skip))]
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

    pub fn register_function(&mut self, id: FunctionId, function: Function) {
        self.functions.entry(id).or_insert(function);
    }

    pub fn register_constant(&mut self, id: ConstantId, constant: Constant) {
        self.constants.entry(id).or_insert(constant);
    }

    pub fn register_service(&mut self, id: ServiceId, service: Service) {
        self.services.entry(id).or_insert(service);
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

    fn register_function(&mut self, id: FunctionId, function: Function) {
        self.functions.entry(id).or_insert(function);
    }

    fn register_constant(&mut self, id: ConstantId, constant: Constant) {
        self.constants.entry(id).or_insert(constant);
    }

    fn register_service(&mut self, id: ServiceId, service: Service) {
        self.services.entry(id).or_insert(service);
    }

    fn register(&mut self, f: impl Fn(&mut Self)) -> &mut Self {
        f(self);
        self
    }
}
