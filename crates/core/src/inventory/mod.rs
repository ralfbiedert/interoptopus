//! Collection of all FFI items a library wants to expose.
//!
//! The main entry point is [`RustInventory`], built with a builder pattern and the
//! registration macros [`function!`](crate::function), [`service!`](crate::service), [`constant!`](crate::constant), and [`extra_type!`](crate::extra_type).
//! Types referenced by registered functions and services are discovered automatically.
//!
//! ```rust
//! use interoptopus::{ffi, function, service, constant};
//! use interoptopus::inventory::RustInventory;
//!
//! # #[ffi] pub enum MyError { General }
//! # #[ffi] pub fn foo(a: u32, b: u32) -> u32 { a + b }
//! # #[ffi] pub const MAX: u32 = 100;
//! # #[ffi(service)] pub struct Service { v: u32 }
//! # #[ffi] impl Service {
//! #     pub fn create() -> ffi::Result<Self, MyError> { ffi::Ok(Self { v: 0 }) }
//! # }
//! pub fn ffi_inventory() -> RustInventory {
//!     RustInventory::new()
//!         .register(function!(foo))
//!         .register(constant!(MAX))
//!         .register(service!(Service))
//!         .validate()
//! }
//! ```
mod id;
mod macros;
mod plugin;
mod rust;

use crate::lang::constant::Constant;
use crate::lang::function::Function;
use crate::lang::service::Service;
use crate::lang::types::Type;
use std::collections::HashMap;

pub use id::{hash_str, ConstantId, FunctionId, Id, PluginId, ServiceId, TypeId};
pub use plugin::PluginInventory;
pub use rust::RustInventory;

/// All registered types.
pub type Types = HashMap<TypeId, Type>;
/// All registered functions.
pub type Functions = HashMap<FunctionId, Function>;
/// All registered constants.
pub type Constants = HashMap<ConstantId, Constant>;
/// All registered services.
pub type Services = HashMap<ServiceId, Service>;

/// Trait implemented by inventory types that can accept FFI item registrations.
///
/// You typically don't need to use this directly — the registration macros
/// ([`function`](crate::function!), [`service`](crate::service!), etc.) call these methods for you.
pub trait Inventory {
    /// Register a type.
    fn register_type(&mut self, id: TypeId, ty: Type);
    /// Register a function.
    fn register_function(&mut self, id: FunctionId, function: Function);
    /// Register a constant.
    fn register_constant(&mut self, id: ConstantId, constant: Constant);
    /// Register a service.
    fn register_service(&mut self, id: ServiceId, service: Service);
    /// Register an item via a closure produced by a registration macro.
    fn register(&mut self, f: impl Fn(&mut Self)) -> &mut Self;
}
