use crate::inventory2::Inventory;
use crate::lang2::constant::ConstantId;
use crate::lang2::function::FunctionId;
use crate::lang2::service::ServiceId;
use crate::lang2::types::TypeId;

pub mod constant;
pub mod function;
pub mod meta;
pub mod service;
pub mod types;

pub trait TypeInfo {
    fn id() -> TypeId;
}

pub trait FunctionInfo {
    fn id() -> FunctionId;
}

pub trait ConstantInfo {
    fn id() -> ConstantId;
}

// TODO:
//
// We'll have to change how #[ffi_type(opaque)] and #[ffi_service] work.
//
// In the future, services types will be #[ffi_type(service)], which will emit
// a `TypeInfo` but not a `Register`, and the `impl` block will still have `#[ffi_service]`
// that emits `ServiceInfo` and `Register`, and registers both the type, and the service.
pub trait ServiceInfo {
    fn id() -> ServiceId;
}

pub trait Register {
    fn register(inventory: &mut Inventory);
}
