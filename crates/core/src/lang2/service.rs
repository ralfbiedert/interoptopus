use crate::inventory2::{FunctionId, ServiceId, TypeId};

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

/// Combines a receiver, constructor, destructor and multiple methods in one entity.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Service {
    ty: TypeId,
    ctors: Vec<FunctionId>,
    destructor: FunctionId,
    methods: Vec<FunctionId>,
}
