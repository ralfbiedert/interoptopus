/// Registers an additional type with the inventory.
///
/// Use this for types that are not already pulled in transitively through a function
/// or service signature but still need to appear in the generated bindings.
///
/// ```rust
/// # use interoptopus::{ffi, extra_type};
/// # use interoptopus::inventory::RustInventory;
/// #[ffi]
/// pub struct Extra { pub value: u32 }
///
/// pub fn ffi_inventory() -> RustInventory {
///     RustInventory::new()
///         .register(extra_type!(Extra))
///         .validate()
/// }
/// ```
#[macro_export]
macro_rules! extra_type {
    ($x:ty) => {{
        use $crate::lang::types::TypeInfo;

        |inventory| {
            <$x as TypeInfo>::register(inventory);
        }
    }};
}

/// Registers an `#[ffi]` function with the inventory.
///
/// The argument is the function's path (the `#[ffi]` attribute generates a companion
/// type that implements [`FunctionInfo`](crate::lang::function::FunctionInfo)).
/// All parameter and return types are registered automatically.
///
/// ```rust
/// # use interoptopus::{ffi, function};
/// # use interoptopus::inventory::RustInventory;
/// #[ffi]
/// pub fn add(a: u32, b: u32) -> u32 { a + b }
///
/// pub fn ffi_inventory() -> RustInventory {
///     RustInventory::new()
///         .register(function!(add))
///         .validate()
/// }
/// ```
#[macro_export]
macro_rules! function {
    ($x:ty) => {{
        |inventory| {
            <$x as $crate::lang::function::FunctionInfo>::register(inventory);
        }
    }};
}

/// Registers an `#[ffi]` constant with the inventory.
///
/// The argument is the constant's path (the `#[ffi]` attribute generates a companion
/// type that implements [`ConstantInfo`](crate::lang::constant::ConstantInfo)).
///
/// ```rust
/// # use interoptopus::{ffi, constant};
/// # use interoptopus::inventory::RustInventory;
/// #[ffi]
/// pub const MAX_ITEMS: u32 = 1024;
///
/// pub fn ffi_inventory() -> RustInventory {
///     RustInventory::new()
///         .register(constant!(MAX_ITEMS))
///         .validate()
/// }
/// ```
#[macro_export]
macro_rules! constant {
    ($x:ty) => {{
        |inventory| {
            <$x as $crate::lang::constant::ConstantInfo>::register(inventory);
        }
    }};
}

/// Registers an `#[ffi]` service with the inventory.
///
/// The argument is the service struct's path. The struct must have both `#[ffi(service)]`
/// on the struct and `#[ffi]` on its `impl` block. All methods and their types are
/// registered automatically.
///
/// ```rust
/// # use interoptopus::{ffi, service};
/// # use interoptopus::inventory::RustInventory;
/// #[ffi]
/// pub enum MyError { General }
///
/// #[ffi(service)]
/// pub struct MyService { value: u32 }
///
/// #[ffi]
/// impl MyService {
///     pub fn new() -> ffi::Result<Self, MyError> { ffi::Ok(Self { value: 0 }) }
///     pub fn get(&self) -> u32 { self.value }
/// }
///
/// pub fn ffi_inventory() -> RustInventory {
///     RustInventory::new()
///         .register(service!(MyService))
///         .validate()
/// }
/// ```
#[macro_export]
macro_rules! service {
    ($x:ty) => {{
        |inventory| {
            <$x as $crate::lang::service::ServiceInfo>::register(inventory);
        }
    }};
}
