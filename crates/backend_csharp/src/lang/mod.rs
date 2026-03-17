#![doc(hidden)]
//! The core C# language model we understand (types, functions, ...).

pub mod constant;
pub mod functions;
mod id;
pub mod meta;
pub mod pattern;
pub mod service;
pub mod types;

pub use id::{ConstantId, FunctionId, ServiceId, TypeId};
