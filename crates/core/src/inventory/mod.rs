mod id;
mod macros;

pub use id::{ConstantId, FunctionId, Id, ServiceId, TypeId, hash_str};

use crate::lang::constant::Constant;
use crate::lang::function::Function;
use crate::lang::service::Service;
use crate::lang::types::Type;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::mem::swap;

pub trait Inventory {
    fn register_type(&mut self, id: TypeId, ty: Type);
    fn register_function(&mut self, id: FunctionId, function: Function);
    fn register_constant(&mut self, id: ConstantId, constant: Constant);
    fn register_service(&mut self, id: ServiceId, service: Service);
    fn register(&mut self, f: impl Fn(&mut Self)) -> &mut Self;
}

// TODO: This should be 2 models: `RustInventory` & `ForeignInventory`
#[derive(Default)]
pub struct RustInventory {
    pub types: HashMap<TypeId, Type>,
    pub functions: HashMap<FunctionId, Function>,
    pub constants: HashMap<ConstantId, Constant>,
    pub services: HashMap<ServiceId, Service>,
    _guard: PhantomData<()>,
}

impl RustInventory {
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

impl Inventory for RustInventory {
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
