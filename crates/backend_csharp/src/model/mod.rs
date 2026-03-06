use crate::lang::constant::Constant;
use crate::lang::function::Function;
use crate::lang::service::Service;
use crate::lang::types::Type;
use std::collections::HashMap;
use std::marker::PhantomData;

mod id;

pub use id::{ConstantId, FunctionId, ServiceId, TypeId};

pub type Types = HashMap<TypeId, Type>;
pub type Functions = HashMap<FunctionId, Function>;
pub type Constants = HashMap<ConstantId, Constant>;
pub type Services = HashMap<ServiceId, Service>;

#[derive(Default)]
pub struct RustPluginModel {
    pub types: Types,
    pub functions: Functions,
    pub constants: Constants,
    pub services: Services,
    _guard: PhantomData<()>,
}

// TODO: For later
#[derive(Default)]
pub struct CsharpPluginModel {
    pub types: HashMap<TypeId, Type>,
    _guard: PhantomData<()>,
}
