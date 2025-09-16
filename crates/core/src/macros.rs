/// Register a function with an [`InventoryBuilder`](crate::inventory::InventoryBuilder).
///
/// You must also annotate the function with [`#[ffi_function]`](crate::ffi_function).
///
/// # Example
///
/// ```rust
/// use interoptopus::{ffi_function, function};
/// use interoptopus::inventory::{Inventory, InventoryBuilder};
///
/// #[ffi_function]
/// pub fn my_function() { }
///
/// pub fn inventory() -> Inventory {
///     Inventory::builder()
///         .register(function!(my_function))
///         .validate()
///         .build()
/// }
/// ```
#[macro_export]
macro_rules! function {
    ($x:ty) => {{
        use $crate::lang::FunctionInfo;
        // use $x as fnc;
        let info = <$x>::function_info();
        $crate::inventory::Symbol::Function(info)
    }};
}

/// Register an extra type with an [`InventoryBuilder`](crate::inventory::InventoryBuilder).
///
/// You must also annotate the type with [`#[ffi_type]`](crate::ffi_type) and `#[repr(C)]`.
///
/// Note, most types are registered automatically. You only need this to pass types not directly visible in
/// your function signatures, e.g., when accepting multiple types via a void pointer.
///
/// # Example
///
/// ```rust
/// use interoptopus::{ffi_type, extra_type};
/// use interoptopus::inventory::{Inventory, InventoryBuilder};
///
/// #[ffi_type]
/// pub struct S<T> {
///     t: T
/// };
///
/// pub fn inventory() -> Inventory {
///     Inventory::builder()
///         .register(extra_type!(S<f32>))
///         .validate()
///         .build()
/// }
#[macro_export]
macro_rules! extra_type {
    ($x:ty) => {{
        let info = <$x as $crate::lang::TypeInfo>::type_info();
        $crate::inventory::Symbol::Type(info)
    }};
}

/// Register a pattern with an [`InventoryBuilder`](crate::inventory::InventoryBuilder).
///
/// You only need to register [`LibraryPattern`](crate::pattern::LibraryPattern), as [`TypePattern`](crate::pattern::TypePattern) are detected automatically.
///
/// ```rust
/// use interoptopus::{ffi, ffi_type, ffi_service, pattern};
/// use interoptopus::inventory::{InventoryBuilder, Inventory};
///
/// # use std::fmt::{Display, Formatter};
/// #
/// # impl Display for Error {
/// #     fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
/// #         Ok(())
/// #     }
/// # }
/// #
/// # impl std::error::Error for Error {}
/// #
/// # #[ffi_type]
/// # #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
/// # pub enum Error {
/// #     Bad,
/// # }
/// #
///
/// #[ffi_type(opaque)]
/// pub struct SimpleService {
///     pub some_value: u32,
/// }
///
/// #[ffi_service]
/// impl SimpleService {
///
///     pub fn new_with(some_value: u32) -> ffi::Result<Self, Error> {
///         ffi::Ok(Self { some_value })
///     }
/// }
///
/// pub fn inventory() -> Inventory {
///     Inventory::builder()
///         .register(pattern!(SimpleService))
///         .validate()
///         .build()
/// }
#[macro_export]
macro_rules! pattern {
    ($x:path) => {{
        let info: $crate::pattern::LibraryPattern = <$x as $crate::pattern::LibraryPatternInfo>::pattern_info();
        $crate::inventory::Symbol::Pattern(info)
    }};
}

/// Register a constant with an [`InventoryBuilder`](crate::inventory::InventoryBuilder).
///
/// You must also annotate the constant with [`#[ffi_constant]`](crate::ffi_constant).
///
/// # Example
///
/// ```rust
/// use interoptopus::{ffi_constant, constant};
/// use interoptopus::inventory::{Inventory, InventoryBuilder};
///
/// #[ffi_constant]
/// pub const MY_CONSTANT: u32 = 123;
///
/// pub fn inventory() -> Inventory {
///     Inventory::builder()
///         .register(constant!(MY_CONSTANT))
///         .validate()
///         .build()
/// }
/// ```
#[macro_export]
macro_rules! constant {
    ($x:path) => {{
        use $crate::lang::ConstantInfo;
        use $x as constant;
        let info = constant::constant_info();
        $crate::inventory::Symbol::Constant(info)
    }};
}

/// Generate FFI plugin structure and functions.
///
/// This macro parses plugin definitions with functions and traits.
/// You can fill in the implementation details as needed.
///
/// # Example
///
/// ```rust
/// use interoptopus::{ffi_plugin, ffi};
///
/// ffi_plugin!(Blah {
///     fn foo(x: Something) -> ffi::String;
///
///     trait Foo {
///         fn bar(&self) -> ffi::String;
///         fn bar(&self) -> ffi::String;
///     }
/// });
/// ```
#[macro_export]
macro_rules! ffi_plugin {
    ($plugin_name:ident {
        $(fn $fn_name:ident($($param:ident: $param_type:ty),*) -> $ret_type:ty;)*
        
        $(trait $trait_name:ident {
            $(fn $method_name:ident(&self$(, $method_param:ident: $method_param_type:ty)*) -> $method_ret_type:ty;)*
        })*
    }) => {
        // TODO: Add your implementation here
        // The macro successfully parses:
        // - Plugin name: $plugin_name
        // - Functions: $fn_name with params and return types
        // - Traits: $trait_name with methods
        
        // compile_error!("ffi_plugin! macro shell - implement your logic here");
    };
}
