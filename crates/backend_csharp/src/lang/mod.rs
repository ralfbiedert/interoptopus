use std::collections::HashMap;

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
