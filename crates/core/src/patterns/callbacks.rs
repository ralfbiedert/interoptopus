//! Useful when `extern "C" fn()` delegate types give compile errors.
//!
//! # Example
//!
//! If you want to accept user-provided callbacks or "delegates":
//!
//!```
//! use interoptopus::{ffi_function, callback};
//! use interoptopus::patterns::slice::FFISlice;
//!
//! callback!(CallbackSlice(x: FFISlice<u8>) -> u8);
//!
//! #[ffi_function]
//! pub fn my_function(callback: CallbackSlice) {
//!     callback.call(FFISlice::empty());
//! }
//!
//! ```
//! Backends supporting this pattern might generate the equivalent to the following pseudo-code:
//!
//! ```csharp
//! public delegate uint CallbackSlice(Sliceu8 x0);
//!
//! void my_function(CallbackSlice callback);
//! ```
//!
//! Backends not supporting this pattern, and C FFI, will see the equivalent of the following C code:
//! ```c
//! typedef void (*fptr_fn_Sliceu8)(my_library_slicemutu8 x0);
//!
//! void my_function(fptr_fn_Sliceu8 callback);
//! ```
//!
//!
//! # Code Generation
//!
//! The macro [**`callback`**](crate::callback) enables two use cases:
//!
//! - On the **Rust** side it will generate a new function-pointer type with better compatibility
//!   with respect to lifetimes in signatures, and accepting an unlimited number of args.
//! - On the **FFI side** a _properly named_ callback (delegate, function pointer ...) type can be
//!   produced (e.g., `MyCallback`), instead of one where it's name is just a generic concatenation
//!   of all used parameters (e.g., `InteropDelegate_fn_i32_i32`).
//!
//!
//! # Why we need the macro `callback!`
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
//! # How to return callbacks from functions
//!
//! Due to another Rust limitation this won't work, despite the `From<>` conversion
//! "being implemented".
//!
//! ```rust,ignore
//! use interoptopus::{ffi_function, callback};
//!
//! callback!(SumFunction(x: i32, y: i32) -> i32);
//!
//! #[ffi_function]
//! #[no_mangle]
//! pub extern "C" fn return_sum_function() -> SumFunction {
//!     my_sum_function.into() // Compile error, mismatch between `function item type` and `function pointer type`
//! }
//!
//! extern "C" fn my_sum_function(x: i32, y: i32) -> i32 { x + y }
//! ```
//!
//! Instead, you will have to return function pointers like so:
//!
//! ```rust
//! # use interoptopus::{ffi_function, callback};
//! # callback!(SumFunction(x: i32, y: i32) -> i32);
//! # extern "C" fn my_sum_function(x: i32, y: i32, _: *const std::ffi::c_void) -> i32 { x + y }
//! #
//! #[ffi_function]
//! pub fn return_sum_function() -> SumFunction {
//!     SumFunction(Some(my_sum_function), std::ptr::null())
//! }
//! ```

use crate::lang::c::{CType, FnPointerType, Meta};
use std::ops::Deref;

/// Internal helper naming a generated callback type wrapper.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NamedCallback {
    fnpointer: FnPointerType,
    meta: Meta,
}

impl NamedCallback {
    /// Creates a new named callback.
    #[must_use]
    pub fn new(callback: FnPointerType) -> Self {
        Self::with_meta(callback, Meta::new())
    }

    /// Creates a new named callback with the given meta.
    ///
    /// # Panics
    ///
    /// The provided pointer must have a name.
    #[must_use]
    pub fn with_meta(callback: FnPointerType, meta: Meta) -> Self {
        assert!(callback.name().is_some(), "The pointer provided to a named callback must have a name.");
        Self { fnpointer: callback, meta }
    }

    /// Gets the type name of this callback.
    ///
    /// # Panics
    ///
    /// Assumes the given pointer has a name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.fnpointer.name().unwrap()
    }

    /// Gets the type's meta.
    #[must_use]
    pub const fn meta(&self) -> &Meta {
        &self.meta
    }

    /// Returns the function pointer type.
    #[must_use]
    pub const fn fnpointer(&self) -> &FnPointerType {
        &self.fnpointer
    }
}

/// Helper naming a (hidden) async callback trampoline.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct AsyncCallback {
    fnpointer: FnPointerType,
    meta: Meta,
}

impl AsyncCallback {
    /// Creates a new async callback.
    #[must_use]
    pub fn new(callback: FnPointerType) -> Self {
        Self::with_meta(callback, Meta::new())
    }

    /// Creates a new async callback with the given meta.
    ///
    /// # Panics
    ///
    /// The provided pointer must have a name.
    #[must_use]
    pub fn with_meta(callback: FnPointerType, meta: Meta) -> Self {
        assert!(callback.name().is_some(), "The pointer provided to a named callback must have a name.");
        Self { fnpointer: callback, meta }
    }

    /// Gets the type's meta.
    #[must_use]
    pub fn target(&self) -> &CType {
        self.fnpointer
            .signature()
            .params()
            .first()
            .expect("Must have a first parameter")
            .the_type()
            .pointer_target()
            .expect("Paramter must be a pointer")
    }

    /// Gets the type's meta.
    #[must_use]
    pub const fn meta(&self) -> &Meta {
        &self.meta
    }

    /// Returns the function pointer type.
    #[must_use]
    pub const fn fnpointer(&self) -> &FnPointerType {
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
///
/// You can also create the callback from Rust for testing:
///
/// ```
/// use interoptopus::callback;
///
/// callback!(MyCallback() -> u8);
///
/// extern "C" fn my_rust_callback(_: *const std::ffi::c_void) -> u8 {
///     42
/// }
///
/// let callback = MyCallback::new(my_rust_callback);
/// assert_eq!(42, callback.call());
/// ```
#[macro_export]
macro_rules! callback {
    ($name:ident($($param:ident: $ty:ty),*)) => {
        callback!($name($($param: $ty),*) -> ());
    };

    ($name:ident($($param:ident: $ty:ty),*) -> $rval:ty $(, namespace = $ns:expr)?) => {
        #[derive(Clone, Copy)]
        #[repr(C)]
        pub struct $name(::std::option::Option<extern "C" fn($($ty,)* *const ::std::ffi::c_void) -> $rval>, *const ::std::ffi::c_void);

        impl $name {
            /// Creates a new instance of the callback using `extern "C" fn`
            pub fn new(func: extern "C" fn($($ty,)* *const ::std::ffi::c_void) -> $rval) -> Self {
                Self(Some(func), ::std::ptr::null())
            }

            /// Will call function if it exists, panic otherwise.
            pub fn call(&self, $($param: $ty),*) -> $rval {
                self.0.expect("Assumed function would exist but it didn't.")($($param,)* self.1)
            }

            /// Will call function only if it exists
            pub fn call_if_some(&self, $($param: $ty,)*) -> ::std::option::Option<$rval> {
                match self.0 {
                    Some(c) => Some(c($($param,)* self.1)),
                    None => None
                }
            }
        }

        impl From<for<> extern "C" fn($($ty,)* *const ::std::ffi::c_void) -> $rval> for $name {
            fn from(x: extern "C" fn($($ty,)* *const ::std::ffi::c_void) -> $rval) -> Self {
                Self(Some(x), ::std::ptr::null())
            }
        }

        impl From<$name> for ::std::option::Option<extern "C" fn($($ty,)* *const ::std::ffi::c_void) -> $rval> {
            fn from(x: $name) -> Self {
                x.0
            }
        }

        unsafe impl $crate::lang::rust::CTypeInfo for $name {
            fn type_info() -> $crate::lang::c::CType {
                use $crate::lang::rust::CTypeInfo;
                use $crate::lang::c::{CType, Meta, Documentation, PrimitiveType, Parameter, FunctionSignature, FnPointerType};

                let rval = < $rval as CTypeInfo >::type_info();

                let params = vec![
                $(
                    Parameter::new(stringify!($param).to_string(), < $ty as CTypeInfo >::type_info()),
                )*
                    Parameter::new("callback_data".to_string(), CType::ReadPointer(Box::new(CType::Primitive(PrimitiveType::Void)))),
                ];

                let mut namespace = ::std::string::String::new();
                $(
                    namespace = ::std::string::String::from($ns);
                )*

                let meta = Meta::with_namespace_documentation(namespace, Documentation::new());
                let sig = FunctionSignature::new(params, rval);
                let fn_pointer = FnPointerType::new_named(sig, stringify!($name).to_string());
                let named_callback = $crate::patterns::callbacks::NamedCallback::with_meta(fn_pointer, meta);

                CType::Pattern($crate::patterns::TypePattern::NamedCallback(named_callback))
            }
        }
    };
}
