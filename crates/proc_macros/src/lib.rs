//! Proc macros for [Interoptopus](https://github.com/ralfbiedert/interoptopus).
//!
//! Items in here will be re-exported by [the main crate](https://crates.io/crates/interoptopus).

extern crate proc_macro; // Apparently needed to be imported like this.

mod constants;
mod functions;
mod macros;
mod service;
mod types;
mod util;

use proc_macro::TokenStream;

/// Enable a `struct` or `enum` to appear in generated bindings.
///
/// This will derive [`CTypeInfo`](https://docs.rs/interoptopus/latest/interoptopus/lang/rust/trait.CTypeInfo.html) based on the _visible_ information in the type definition. This
/// is the preferred way of enabling FFI types; although in some cases (e.g., when dealing with
/// types outside your control) you will have to implement a **surrogate** manually, see below.
///
/// A number of attributes are available:
///
/// | Attribute | On |  Explanation |
/// | --- | --- | ---  |
/// | `name="X"` | `struct`,`enum` | Uses `name` as the base interop name instead of the item's Rust name.<sup>1</sup> |
/// | `namespace="X"` | `struct`,`enum` | Determine which namespace or file item should go. <sup>2</sup>
/// | `skip(x)` | `struct,enum` | Skip field or variant `x` in the definition, e.g., some `x` of [`PhantomData`](std::marker::PhantomData). <sup>⚠️</sup>
/// | `opaque` | `struct` | Creates an opaque type without fields. Can only be used behind a pointer. <sup>3</sup> |
/// | `transparent` | `struct, enum` | The struct or single variant enum will be `#[repr(transparent)]`. <sup>3</sup> |
/// | `packed` | `struct` | The struct will be `#[repr(packed)]`. <sup>3</sup> |
/// | `error` | `enum` | The enum will follow the `FFIError` result pattern. |
/// | `u8`, ..., `u64` | `enum` | Creates an opaque type without fields. Can only be used behind a pointer. |
/// | `visibility(x="v")` | `struct` | Override visibility for field `x` as `public` or `private`; `_all` means all fields. <sup>2</sup>
/// | `debug` | * | Print generated helper code in console.
///
/// <sup>1</sup> While a type's name must be unique (even across modules) backends are free to further transform this name, e.g., by converting
/// `MyVec` to `LibraryMyVec`. In other words, using `name` will change a type's name, but not using `name` is no guarantee the final name will
/// not be modified.
///
/// <sup>2</sup> Will not be reflected in C backend, but available to languages supporting them,
/// e.g., C# will emit field visibility and generate classes from service patterns.
///
/// <sup>3</sup> If nothing else is specified the resulting type will become `#[repr(C)]` by default.
///
/// # Types and the Inventory
///
/// In contrast to functions and constants most types annotated with `#[ffi_type]` will be detected
/// automatically and need no mention in the inventory function.
///
/// The exception are types that do not show up as fields of another type, or inside a function
/// signature.
///
///
/// # Patterns
///
/// Patterns allow you to write, and backends to generate more idiomatic code. The following
/// patterns are currently supported by this annotation:
///
/// | Pattern | On |  Explanation |
/// | --- | --- | ---  |
/// | `ffi_error` | `enum` | Denotes this as a [`FFIError`](https://docs.rs/interoptopus/latest/interoptopus/patterns/result/trait.FFIError.html). |
///
/// # Examples
///
/// ```
/// use interoptopus::ffi_type;
///
/// #[ffi_type(opaque, name = "MyVec")]
/// #[derive(Copy, Clone, Debug)]
/// pub struct Vec2f32 {
///     pub x: f32,
///     pub y: f32,
///     pub z: f32,
/// }
/// ```
///
#[proc_macro_attribute] // Can now be used as `#[my_attribute]`
pub fn ffi_type(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = proc_macro2::TokenStream::from(attr);
    let input = proc_macro2::TokenStream::from(item);
    types::ffi_type(attr, input).into()
}

/// Enable an `extern "C"` function to appear in generated bindings.
///
/// This will derive [`FunctionInfo`](https://docs.rs/interoptopus/latest/interoptopus/lang/rust/trait.FunctionInfo.html) for a helper struct
/// of the same name containing the function's name, parameters and return value.
///
/// In order to appear in generated bindings the function also has to be mentioned in the inventory function.
///
/// # Parameters
///
/// The following parameters can be provided:
///
/// | Parameter |  Explanation |
/// | --- | ---  |
/// | `debug` | Print generated helper code in console.
///
/// # Safety
///
/// ⚠️ You _must_ ensure that methods exported with `#[ffi_function]` will never panic. We highly encourage you
/// to wrap all your code in panic guards. This is a standard Rust FFI concern and has nothing to do with Interoptopus.
/// Failure to follow this advice will probably lead to undefined behavior down the road. The author has been there and does not recommend it.
///
/// # Example
///
/// ```
/// use interoptopus::ffi_function;
///
/// #[ffi_function]
/// pub fn my_function(x: u32) -> u32 {
///     x
/// }
/// ```
#[proc_macro_attribute] // Can now be used as `#[my_attribute]`
pub fn ffi_function(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = proc_macro2::TokenStream::from(attr);
    let input = proc_macro2::TokenStream::from(item);
    functions::ffi_function(attr, input).into()
}

/// Enables a `const` to appear in generated bindings.
///
/// This will derive [`ConstantInfo`](https://docs.rs/interoptopus/latest/interoptopus/lang/rust/trait.ConstantInfo.html) for a helper struct of the
/// same name containing the const's name and value.
///
/// Constant evaluation is supported.
///
/// In order to appear in generated bindings the constant also has to be mentioned in the inventory function.
///
/// # Examples
///
/// ```
/// use interoptopus::ffi_constant;
/// # const fn double(x: u8) -> u8 { 2 * x }
///
/// #[ffi_constant]
/// const SOME_CONST: u32 = 314;
///
/// #[ffi_constant]
/// const COMPUTED_CONST: u8 = double(12); // will export 24
///
/// ```
#[proc_macro_attribute] // Can now be used as `#[my_attribute]`
pub fn ffi_constant(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = proc_macro2::TokenStream::from(attr);
    let input = proc_macro2::TokenStream::from(item);
    constants::ffi_constant(attr, &input).into()
}

/// Creates an FFI service from an `impl Service {}` block.
///
/// See the [service module](https://docs.rs/interoptopus/latest/interoptopus/patterns/service/index.html) for an introduction into services.
///
/// In order to appear in generated bindings the service also has to be mentioned in the inventory function.
///
/// # Requirements
///
/// For this attribute to work a number of preconditions must be fulfilled:
///
/// - The attribute must be used on `impl SomeType {}` blocks
/// - The `error` parameter must be provided and point to an [`FFIError`](https://docs.rs/interoptopus/latest/interoptopus/patterns/result/trait.FFIError.html) type.
/// - The respective `SomeType` type must have an [`#[ffi_type(opaque)]`](macro@crate::ffi_type) attribute.
///
/// We recommend to have a look at the [reference project](https://github.com/ralfbiedert/interoptopus/blob/master/interoptopus_reference_project/src/patterns/service.rs).
///
/// # Parameters
///
/// The following parameters can be provided:
///
/// | Parameter |  Explanation |
/// | --- | ---  |
/// | `error = "t"` | Use `t` as the [`FFIError`](https://docs.rs/interoptopus/latest/interoptopus/patterns/result/trait.FFIError.html) type, mandatory.
/// | `prefix  = "p"` | Add `p` to all generated method names. If not given the prefix will be inferred from the type.
///
/// # Example
///
/// ```
/// # use std::fmt::{Display, Formatter};
/// # use interoptopus::pattern::result::FFIError;
/// # #[derive(Debug)]
/// # pub enum Error {
/// #     Bad,
/// # }
/// #
/// # impl Display for Error {
/// #     fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
/// #         Ok(())
/// #     }
/// # }
/// #
/// # impl std::error::Error for Error {}
/// #
/// # #[ffi_type(error)]
/// # #[derive(PartialOrd, PartialEq, Debug, Copy, Clone)]
/// # pub enum MyFFIError {
/// #     Ok = 0,
/// #     NullPassed = 1,
/// #     Panic = 2,
/// #     OtherError = 3,
/// # }
/// #
/// # impl FFIError for MyFFIError {
/// #     const SUCCESS: Self = Self::Ok;
/// #     const NULL: Self = Self::NullPassed;
/// #     const PANIC: Self = Self::Panic;
/// # }
/// #
/// # impl From<Error> for MyFFIError {
/// #     fn from(x: Error) -> Self {
/// #         match x {
/// #             Error::Bad => Self::OtherError,
/// #         }
/// #     }
/// # }
/// #
/// use interoptopus::{ffi, ffi_type, ffi_service};
///
/// #[ffi_type(opaque)]
/// pub struct SimpleService { }
///
/// #[ffi_service]
/// impl SimpleService {
///     pub fn new_with(some_value: u32) -> ffi::Result<Self, MyFFIError> {
///         ffi::Ok(Self { })
///     }
/// }
/// ```
///
#[proc_macro_attribute] // Can now be used as `#[my_attribute]`
pub fn ffi_service(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = proc_macro2::TokenStream::from(attr);
    let input = proc_macro2::TokenStream::from(item);
    service::ffi_service(attr, &input).into()
}

/// Inside a [`#[ffi_service]`](macro@crate::ffi_service) block, configure the generated FFI helper.
///
/// This is an optional attribute that can be applied to some methods.
///
/// By default, service methods
/// must return a `Result<(), Error>` return type that will be mapped to an `FFIError` and
/// transparently checked in languages supporting the pattern.
/// However, sometimes you might want to return an actual value. Using this attribute and specifying
/// an `on_panic` behavior allows you to opt out of error mapping, and instead return values as-is.
///
/// See the [service module](https://docs.rs/interoptopus/latest/interoptopus/patterns/service/index.html) for an introduction into services.
///
/// # Parameters
///
/// The following attributes can be provided:
///
/// | Parameter |  Explanation |
/// | --- | ---  |
/// | `ignore` | Don't emit to FFI. |
/// | `on_panic` | Determines what will happen on a panic (`ffi_error`, `return_default`, `undefined_behavior`) and, as a side effect, _also_ determine how return values will be handled. See below. |
///
///
/// ## Wrapping Behavior
///
/// Details what `on_panic` means:
///
/// | Mode |  Explanation |
/// | --- | ---  |
/// | `ffi_error` | Method must return `Result<(), Error>` and maps that to an `FFIError`. Default behavior.
/// | `return_default` | Method can return any `T: Default`. If a panic occurs [`T::default()`](Default::default) will be returned, see below.
/// | `undefined_behavior` | Method can return any `T`. If a panic occurs undefined behavior happens. Slightly faster (nanoseconds) and mostly an escape hatch when running into lifetime issues in autogenerated code, e.g., when returning an `CStrPointer` from a service. In the long term our proc macro code gen should be fixed to handle this situation.
///
/// # Panic Behavior
///
/// ⚠️ Generated methods add panic guards when used with `ffi_error` and `return_default`. However, since `return_default` methods
/// have no other way to signal errors they will return [`Default::default()`] instead if a panic
/// is encountered. If you compiled Interoptopus with the `log` feature a message will be emitted
/// in that case.
///
/// # Safety
///
/// ⚠️ You must ensure that methods marked with `on_panic = "undefined_behavior"` will never panic. Failure to do so will lead to
/// undefined behavior.
///
/// # Example
///
/// ```
/// # use std::fmt::{Display, Formatter};
/// # use interoptopus::pattern::result::FFIError;
/// #
/// # #[derive(Debug)]
/// # pub enum Error {
/// #     Bad,
/// # }
/// #
/// # impl Display for Error {
/// #     fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
/// #         Ok(())
/// #     }
/// # }
/// #
/// # impl std::error::Error for Error {}
/// #
/// # #[ffi_type(error)]
/// # #[derive(PartialOrd, PartialEq, Debug, Copy, Clone)]
/// # pub enum MyFFIError {
/// #     Ok = 0,
/// #     NullPassed = 1,
/// #     Panic = 2,
/// #     OtherError = 3,
/// # }
/// #
/// # impl FFIError for MyFFIError {
/// #     const SUCCESS: Self = Self::Ok;
/// #     const NULL: Self = Self::NullPassed;
/// #     const PANIC: Self = Self::Panic;
/// # }
/// #
/// # impl From<Error> for MyFFIError {
/// #     fn from(x: Error) -> Self {
/// #         match x {
/// #             Error::Bad => Self::OtherError,
/// #         }
/// #     }
/// # }
/// #
/// use interoptopus::{ffi, ffi_type, ffi_service, ffi_service_method};
///
/// #[ffi_type(opaque)]
/// pub struct SimpleService { }
///
/// #[ffi_service]
/// impl SimpleService {
///
///     pub fn new_with(some_value: u32) -> ffi::Result<Self, MyFFIError> {
///         ffi::Ok(Self { })
///     }
///
///     #[allow(unconditional_panic)]
///     pub fn oops(&self, x: u32) -> u32 {
///         let array = vec![0, 1, 2];
///
///         // This will panic. The method will return 0 instead.
///         array[5]
///     }
///
///     pub fn return_value(&self) -> u32 {
///         // If this method ever panicked the entire application calling
///         // you (not just your library) will crash or worse.
///         123
///     }
///
/// }
/// ```
#[proc_macro_attribute]
pub fn ffi_service_method(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
