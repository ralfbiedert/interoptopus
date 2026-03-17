use crate::inventory::{ConstantId, Constants, FunctionId, Functions, Inventory, ServiceId, Services, TypeId, Types};
use crate::lang::constant::Constant;
use crate::lang::function::Function;
use crate::lang::service::Service;
use crate::lang::types::Type;
use std::marker::PhantomData;
use std::mem::swap;

/// The central registry of all FFI items a Rust library exposes.
///
/// Build one using [`RustInventory::new`], chain [`register`](Inventory::register) calls with
/// the registration macros ([`function!`](crate::function), [`service!`](crate::service), [`constant!`](crate::constant), [`extra_type!`](crate::extra_type)),
/// and finalize with [`validate`](RustInventory::validate). The resulting inventory is
/// then passed to a backend to generate bindings.
///
/// # Example
///
/// ```rust
/// use interoptopus::{ffi, function, constant};
/// use interoptopus::inventory::RustInventory;
///
/// #[ffi]
/// pub fn my_add(a: u32, b: u32) -> u32 { a + b }
///
/// #[ffi]
/// pub const MAX: u32 = 100;
///
/// pub fn ffi_inventory() -> RustInventory {
///     RustInventory::new()
///         .register(function!(my_add))
///         .register(constant!(MAX))
///         .validate()
/// }
/// ```
#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RustInventory {
    pub types: Types,
    pub functions: Functions,
    pub constants: Constants,
    pub services: Services,
    #[cfg_attr(feature = "serde", serde(skip))]
    _guard: PhantomData<()>,
}

impl RustInventory {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_type(&mut self, id: TypeId, ty: Type) {
        self.types.entry(id).or_insert(ty);
    }

    pub fn register_function(&mut self, id: FunctionId, function: Function) {
        self.functions.entry(id).or_insert(function);
    }

    pub fn register_constant(&mut self, id: ConstantId, constant: Constant) {
        self.constants.entry(id).or_insert(constant);
    }

    pub fn register_service(&mut self, id: ServiceId, service: Service) {
        self.services.entry(id).or_insert(service);
    }

    #[must_use]
    pub fn register(&mut self, f: impl Fn(&mut Self)) -> &mut Self {
        f(self);
        self
    }

    /// Finalizes the inventory, returning the completed registry.
    ///
    /// Call this as the last step in the builder chain. The returned value is
    /// what you pass to a backend.
    #[must_use]
    pub fn validate(&mut self) -> Self {
        let mut rval = Self::new();
        swap(&mut rval, self);
        rval
    }
}

impl Inventory for RustInventory {
    fn register_type(&mut self, id: TypeId, ty: Type) {
        self.types.entry(id).or_insert(ty);
    }

    fn register_function(&mut self, id: FunctionId, function: Function) {
        self.functions.entry(id).or_insert(function);
    }

    fn register_constant(&mut self, id: ConstantId, constant: Constant) {
        self.constants.entry(id).or_insert(constant);
    }

    fn register_service(&mut self, id: ServiceId, service: Service) {
        self.services.entry(id).or_insert(service);
    }

    fn register(&mut self, f: impl Fn(&mut Self)) -> &mut Self {
        f(self);
        self
    }
}
