extern crate proc_macro; // Apparently needed to be imported like this.

mod constants;
mod functions;
mod types;
mod util;

use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs};

/// Enable a `struct` or `enum` to appear in generated bindings.
///
/// This will derive [`CTypeInfo`](interoptopus::lang::rust::CTypeInfo) based on the _visible_ information in the type definition. This
/// is the preferred way of enabling FFI types; although in some cases (e.g., when dealing with
/// types outside of your control) you will have to implement a **surrogate** manually, see below.
///
/// A number of attributes are available:
///
/// | Attribute | On |  Explanation |
/// | --- | --- | ---  |
/// | `name="X"` | `struct`,`enum` | Uses `name` as the base interop name instead of the item's Rust name.<sup>1</sup> |
/// | `skip(x)` | `struct,enum` | Skip field or variant `x` in the definition, e.g., some `x` of [`PhantomData`](std::marker::PhantomData).
/// | `patterns(p)` | `struct`,`enum` | Mark this type as part of a pattern, see below.
/// | `opaque` | `struct` | Creates an opaque type without fields. Can only be used behind a pointer. |
/// | `surrogates(x="y")` | `struct` | Invoke function `y` to provide a [`CTypeInfo`](interoptopus::lang::rust::CTypeInfo) for field `x`, see below.
///
/// <sup>1</sup> While a type's name must be unique (even across modules) backends are free to further transform this name, e.g., by converting
/// `MyVec` to `LibraryMyVec`. In other words, using `name` will change a type's name, but not using `name` is no guarantee the final name will
/// not be modified.
///
/// In contrast to functions and constants types annotated with `#[ffi_type]` will be detected
/// automatically and do not have to be explicitly mentioned for the definition of the `inventory_function!()`.
///
/// # Surrogates
///
/// When dealing with types outside of your control you will not be able to implement [`CTypeInfo`](interoptopus::lang::rust::CTypeInfo) for them.
/// Instead you need a **surrogate**, a helper function which returns that info for the type.
///
/// The surrogate's signature is:
///
/// ```rust
/// # use interoptopus::lang::c::CType;
/// fn some_foreign_type() -> CType {
///     // Return an appropriate CType
///     # interoptopus::lang::c::CType::Primitive(interoptopus::lang::c::PrimitiveType::U8)
/// }
/// ```
///
/// # Patterns
///
/// Patterns allow you to write, and backends to generate more idiomatic code. The following
/// patterns are currently supported by this annotation:
///
/// | Pattern | On |  Explanation |
/// | --- | --- | ---  |
/// | `success_enum` | `enum` | Denotes this as a [`SuccessEnum`](interoptopus::patterns::successenum::SuccessEnum). |
///
/// # Examples
///
/// ```
/// use interoptopus::ffi_type;
///
/// #[ffi_type(opaque, name = "MyVec")]
/// #[derive(Copy, Clone, Debug)]
/// #[repr(C)]
/// pub struct Vec2f32 {
///     pub x: f32,
///     pub y: f32,
///     pub z: f32,
/// }
/// ```
///
#[proc_macro_attribute] // Can now be used as `#[my_attribute]`
pub fn ffi_type(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(item);
    let attr_args = parse_macro_input!(attr as AttributeArgs);

    let rval = types::ffi_type(attr_args, input);

    rval.into()
}

/// Enable an `extern "C"` function to appear in generated bindings.
///
/// This will derive [`FunctionInfo`](interoptopus::lang::rust::FunctionInfo) for a helper struct
/// of the same name containing the function's name, parameters and return value.
///
/// The following attributes can be provided:
///
/// | Attribute |  Explanation |
/// | --- | ---  |
/// | `surrogates(x="y")` | Invoke function `y` to provide a [`CTypeInfo`](interoptopus::lang::rust::CTypeInfo) for parameter `x`, see below.
///
/// # Surrogates
///
/// When dealing with types outside of your control you will not be able to implement [`CTypeInfo`](interoptopus::lang::rust::CTypeInfo) for them.
/// Instead you need a **surrogate**, a helper function which returns that info for the type.
///
/// The surrogate's signature is:
///
/// ```rust
/// # use interoptopus::lang::c::CType;
/// fn some_foreign_type() -> CType {
///     // Return an appropriate CType
///     # interoptopus::lang::c::CType::Primitive(interoptopus::lang::c::PrimitiveType::U8)
/// }
/// ```
///
/// # Example
///
/// ```
/// use interoptopus::ffi_function;
///
/// #[ffi_function]
/// #[no_mangle]
/// pub extern "C" fn my_function(x: u32) -> u32 {
///     x
/// }
/// ```
#[proc_macro_attribute] // Can now be used as `#[my_attribute]`
pub fn ffi_function(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(item);
    let attr_args = parse_macro_input!(attr as AttributeArgs);

    let rval = functions::ffi_function(attr_args, input);

    rval.into()
}

/// Enables a `const` to appear in generated bindings.
///
/// This will derive [`ConstantInfo`](interoptopus::lang::rust::ConstantInfo) for a helper struct of the
/// same name containing the const's name and value.
///
/// Constant evaluation is supported.
///
/// In order to appear in generated bindings the constant has to be mentioned in the definition
/// of the libaries `inventory_function!()`.
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
    let input = proc_macro2::TokenStream::from(item);
    let attr_args = parse_macro_input!(attr as AttributeArgs);

    let rval = constants::ffi_constant(attr_args, input);

    rval.into()
}
