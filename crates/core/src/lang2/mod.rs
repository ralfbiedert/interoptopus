use crate::inventory2::{ConstantId, FunctionId, Inventory, ServiceId, TypeId};

mod constant;
mod function;
mod meta;
mod service;
pub mod types;

pub use constant::{Constant, ConstantValue};
pub use function::{Argument, Function, Signature};
pub use meta::{Docs, Emission, Visibility};
pub use service::Service;

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

#[macro_export]
macro_rules! type_id {
    ($t:ty) => {{
        use $crate::inventory2::hash_str;

        let t_name = ::std::any::type_name::<$t>();
        let base = $crate::inventory2::TypeId::new(hash_str(t_name));
        let crate_hash = hash_str(env!("CARGO_PKG_NAME"));
        let file_hash = hash_str(file!());
        let line_hash = line!() as u128;

        base.derive(crate_hash).derive(file_hash).derive(line_hash)
    }};
}
