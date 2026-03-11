use std::collections::HashMap;
use std::marker::PhantomData;

pub mod constant;
pub mod functions;
pub mod id;
pub mod meta;
pub mod pattern;
pub mod service;
pub mod types;

pub use id::{ConstantId, FunctionId, ServiceId, TypeId};

pub type Types = HashMap<TypeId, types::Type>;
pub type Functions = HashMap<FunctionId, functions::Function>;
pub type Constants = HashMap<ConstantId, constant::Constant>;
pub type Services = HashMap<ServiceId, service::Service>;

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
    pub types: HashMap<TypeId, types::Type>,
    _guard: PhantomData<()>,
}
