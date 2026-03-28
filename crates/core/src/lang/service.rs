//! FFI service (class-like) definitions.

use crate::inventory::{FunctionId, Inventory, ServiceId, TypeId};

/// Implemented by service structs annotated with `#[ffi(service)]` and `#[ffi] impl`.
///
/// You do not implement this manually — the `#[ffi]` attribute on an `impl` block
/// generates this implementation.
///
/// # Safety
///
/// The metadata returned by this trait drives code generation for
/// class-like service types and the vtable-style dispatch used at runtime.
/// A wrong `id()`, incorrect `service()` description, or misregistered
/// functions will corrupt the generated interface and cause undefined
/// behaviour during FFI calls into or out of the service.
pub unsafe trait ServiceInfo {
    /// The unique identifier for this service.
    fn id() -> ServiceId;
    /// Returns the full service description.
    fn service() -> Service;
    /// Registers this service (and all referenced functions and types) with the given inventory.
    fn register(inventory: &mut impl Inventory);
}

/// A service definition that maps to a class-like construct in target languages.
///
/// Combines constructors, a destructor, and methods around a single opaque type.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Service {
    /// The opaque type backing this service.
    pub ty: TypeId,
    /// Constructor functions (return `ffi::Result<Self, E>`).
    pub ctors: Vec<FunctionId>,
    /// The destructor function.
    pub destructor: FunctionId,
    /// Regular methods.
    pub methods: Vec<FunctionId>,
}

impl Service {
    #[must_use]
    pub fn new(ty: TypeId, ctors: Vec<FunctionId>, destructor: FunctionId, methods: Vec<FunctionId>) -> Self {
        Self { ty, ctors, destructor, methods }
    }
}
