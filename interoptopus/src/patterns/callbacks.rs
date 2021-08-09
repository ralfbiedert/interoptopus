//! Useful when `extern "C" fn()` delegate types give compile errors.
//!
//! # Code Generation
//!
//! The macro [**`callback`**](crate::callback) enables two use cases:
//!
//! - On the **Rust** side it will generate a new function-pointer type with better compatibility
//! with respect to lifetimes in signatures, and accepting an unlimited number of args.
//!- On the **FFI side** a _properly named_ callback (delegate, function pointer ...) type can be
//! produced (e.g., `MyCallback`), instead of one where it's name is just a generic concatenation
//! of all used parameters (e.g., `InteropDelegate_fn_i32_i32`).
//!
//!
//! # Background
//!
//! Due to how we generate FFI metadata and how Rust traits work there are some types which
//! don't work nicely with Interoptopus: function pointers. Practically speaking, the following code _should_ work:
//!
//! ```ignore
//! use interoptopus::ffi_function;
//! use interoptopus::patterns::slice::FFISlice;
//!
//! pub type CallbackSlice = extern "C" fn(FFISlice<u8>);
//!
//! #[ffi_function]
//! pub extern "C" fn my_function(callback: CallbackSlice) {
//!     callback(FFISlice::empty());
//! }
//!
//! ```
//!
//! The intention is to provide a FFI function `my_function`, accepting
//! a callback, which in turn accepts an `FFISlice<'a, u8>`.
//! Although this is valid FFI code to write, a compile error is returned, which may look like this:
//!
//! ```text
//! error: implementation of `CTypeInfo` is not general enough
//!    [...]
//!    = note: ...`CTypeInfo` would have to be implemented for the type `for<'r> extern "C" fn(FFISlice<'r, u8>)`
//!    = note: ...but `CTypeInfo` is actually implemented for the type `extern "C" fn(FFISlice<'0, u8>)`, for some specific lifetime `'0`
//!    = note: this error originates in an attribute macro (in Nightly builds, run with -Z macro-backtrace for more info)
//! ```
//!
//! The reasons for this are somewhat technical, but it boils down to us being unable to generally
//! implement [`CTypeInfo`](crate::lang::rust::CTypeInfo) for _all_ types you may want to use;
//! [`FFISlice`](crate::patterns::slice::FFISlice) here being one of them.
//! To fix this, you can replace `pub type CallbackSlice = ...` with a `callback!` call
//! which should generate a helper type that works.
//!
//! # Example
//!
//! Here is how the example above would be expressed instead:
//!
//!```
//! use interoptopus::{ffi_function, callback};
//! use interoptopus::patterns::slice::FFISlice;
//!
//! callback!(CallbackSlice(x: FFISlice<u8>) -> u8);
//!
//! #[ffi_function]
//! pub extern "C" fn my_function(callback: CallbackSlice) {
//!     callback.call(FFISlice::empty());
//! }
//! ```

use crate::lang::c::FnPointerType;

/// Internal helper naming a generated callback type wrapper.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NamedCallback {
    name: String,
    fnpointer: FnPointerType,
}

impl NamedCallback {
    /// Creates a new named callback.
    pub fn new(name: String, callback: FnPointerType) -> Self {
        Self { name, fnpointer: callback }
    }

    /// Gets the type name of this callback.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the function pointer type.
    pub fn fnpointer(&self) -> &FnPointerType {
        &self.fnpointer
    }
}

/// Defines a callback type, akin to a `fn f(T) -> R` wrapped in an [Option](std::option).
///
/// A named delegate will be emitted in languages supporting them, otherwise a regular
/// function pointer. For details, please see the [**callbacks module**](crate::patterns::callbacks).
///
/// # Example
///
/// This defines a type `MyCallback` with a parameter `slice` returning an `u8`.
///
/// ```
/// use interoptopus::callback;
/// use interoptopus::patterns::slice::FFISlice;
///
/// callback!(MyCallback(slice: FFISlice<u8>) -> u8);
/// ```
///
/// The generated type definition similar to:
///
/// ```
/// # use interoptopus::patterns::slice::FFISlice;
/// #[repr(transparent)]
/// pub struct MyCallback(Option<extern "C" fn(FFISlice<u8>) -> u8>);
/// ```
#[macro_export]
macro_rules! callback {
    ($name:ident($($param:ident: $ty:ty),*) $(-> $rval:ty)?) => {
        #[derive(Clone)]
        #[repr(transparent)]
        pub struct $name(Option<extern "C" fn($($ty),*) $(-> $rval)?>);

        impl $name {
            /// Will call function if it exists, panic otherwise.
            pub fn call(&self, $($param: $ty),*) $(-> $rval)? {
                self.0.expect("Assumed function would exist but it didn't.")($($param),*)
            }
        }

        impl From<extern "C" fn($($ty),*) $(-> $rval)?> for $name {
            fn from(x: extern "C" fn($($ty),*) $(-> $rval)?) -> Self {
                Self(Some(x))
            }
        }

        unsafe impl interoptopus::lang::rust::CTypeInfo for $name {
            fn type_info() -> interoptopus::lang::c::CType {
                use interoptopus::lang::rust::CTypeInfo;

                let rval = < () as CTypeInfo >::type_info();

                $(
                    let rval = < $rval as CTypeInfo >::type_info();
                )?

                let params = vec![
                $(
                    interoptopus::lang::c::Parameter::new(stringify!($param).to_string(), < $ty as CTypeInfo >::type_info()),
                )*
                ];

                let sig = interoptopus::lang::c::FunctionSignature::new(params, rval);
                let fn_pointer = interoptopus::lang::c::FnPointerType::new(sig);
                let named_callback = interoptopus::patterns::callbacks::NamedCallback::new(stringify!($name).to_string(), fn_pointer);

                interoptopus::lang::c::CType::Pattern(interoptopus::patterns::TypePattern::NamedCallback(named_callback))
            }
        }
    };
}
