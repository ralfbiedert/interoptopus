extern crate proc_macro; // Apparently needed to be imported like this.

mod constants;
mod functions;
mod types;
mod util;

use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs};

/// Enable a type (struct, enum) to appear in generated bindings.
///
/// This will derive `CTypeInfo` based on the 'visible' information in the type definition.
///
/// # Example
///
/// ```
/// use interoptopus::ffi_type;
///
/// #[ffi_type]
/// #[derive(Copy, Clone, Debug)]
/// #[repr(C)]
/// pub struct Vec2f32 {
///     pub x: f32,
///     pub y: f32,
///     pub z: f32,
/// }
/// ```
#[proc_macro_attribute] // Can now be used as `#[my_attribute]`
pub fn ffi_type(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(item);
    let attr_args = parse_macro_input!(attr as AttributeArgs);

    let rval = types::ffi_type(attr_args, input);

    rval.into()
}

/// Enable an `extern "C"` function to appear in generated bindings.
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

/// Enable an `const` to appear in generated bindings.
///
/// # Example
///
/// ```
/// use interoptopus::ffi_constant;
///
/// #[ffi_constant]
/// const MY_CONST: u32 = 314;
/// ```
#[proc_macro_attribute] // Can now be used as `#[my_attribute]`
pub fn ffi_constant(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(item);
    let attr_args = parse_macro_input!(attr as AttributeArgs);

    let rval = constants::ffi_constant(attr_args, input);

    rval.into()
}
