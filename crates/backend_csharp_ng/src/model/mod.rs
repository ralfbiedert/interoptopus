use crate::lang::constant::Constant;
use crate::lang::function::Function;
use crate::lang::service::Service;
use crate::lang::types::Type;
use std::collections::HashMap;
use std::marker::PhantomData;

mod id;

pub use id::{ConstantId, FunctionId, ServiceId, TypeId};

#[derive(Default)]
pub struct RustModel {
    pub types: HashMap<TypeId, Type>,
    pub functions: HashMap<FunctionId, Function>,
    pub constants: HashMap<ConstantId, Constant>,
    pub services: HashMap<ServiceId, Service>,
    _guard: PhantomData<()>,
}

// TODO: For later
#[derive(Default)]
pub struct CsharpModel {
    pub types: HashMap<TypeId, Type>,
    _guard: PhantomData<()>,
}
