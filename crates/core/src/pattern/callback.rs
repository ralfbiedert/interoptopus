//! Named delegates in higher languages.
//!
//! # Example
//!
//! If you want to accept user-provided callbacks or "delegates":
//!
//!```
//! use interoptopus::{ffi_function, callback};
//! use interoptopus::pattern::slice::Slice;
//!
//! callback!(CallbackSlice(x: Slice<u8>) -> u8);
//!
//! #[ffi_function]
//! pub fn my_function(callback: CallbackSlice) {
//!     callback.call(Slice::empty());
//! }
//!
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
//! use interoptopus::pattern::slice::FFISlice;
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
//! implement [`TypeInfo`](crate::lang::TypeInfo) for _all_ types you may want to use;
//! [`FFISlice`](crate::pattern::slice::Slice) here being one of them.
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

use crate::lang::{FnPointer, Meta, Type};

/// Internal helper naming a generated callback type wrapper.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NamedCallback {
    fnpointer: FnPointer,
    meta: Meta,
}

impl NamedCallback {
    /// Creates a new named callback.
    #[must_use]
    pub fn new(callback: FnPointer) -> Self {
        Self::with_meta(callback, Meta::new())
    }

    /// Creates a new named callback with the given meta.
    ///
    /// # Panics
    ///
    /// The provided pointer must have a name.
    #[must_use]
    pub fn with_meta(callback: FnPointer, meta: Meta) -> Self {
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
    pub const fn fnpointer(&self) -> &FnPointer {
        &self.fnpointer
    }
}

/// Helper naming a (hidden) async callback trampoline.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct AsyncCallback {
    fnpointer: FnPointer,
    meta: Meta,
}

impl AsyncCallback {
    /// Creates a new async callback.
    #[must_use]
    pub fn new(callback: FnPointer) -> Self {
        Self::with_meta(callback, Meta::new())
    }

    /// Creates a new async callback with the given meta.
    ///
    /// # Panics
    ///
    /// The provided pointer must have a name.
    #[must_use]
    pub fn with_meta(callback: FnPointer, meta: Meta) -> Self {
        assert!(callback.name().is_some(), "The pointer provided to a named callback must have a name.");
        Self { fnpointer: callback, meta }
    }

    /// Gets the async type's target.
    #[must_use]
    pub fn t(&self) -> &Type {
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
    pub const fn fnpointer(&self) -> &FnPointer {
        &self.fnpointer
    }
}

/// Defines a callback type, akin to a `fn f(T) -> R` wrapped in an [`Option`](std::option).
///
/// A named delegate will be emitted in languages supporting them, otherwise a regular
/// function pointer. For details, please see the [**callbacks module**](crate::pattern::callback).
///
/// # Example
///
/// This defines a type `MyCallback` with a parameter `slice` returning an `u8`.
///
/// ```
/// use interoptopus::callback;
/// use interoptopus::pattern::slice::Slice;
///
/// callback!(MyCallback(slice: Slice<u8>) -> u8);
/// ```
///
/// The generated type definition similar to:
///
/// ```
/// # use interoptopus::pattern::slice::Slice;
/// #[repr(transparent)]
/// pub struct MyCallback(Option<extern "C" fn(Slice<u8>) -> u8>);
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
        #[derive(Default, Clone, Copy)]
        #[repr(C)]
        pub struct $name(::std::option::Option<extern "C" fn($($ty,)* *const ::std::ffi::c_void) -> $rval>, *const ::std::ffi::c_void);

        // Safety: This is a transparent wrapper around a function pointer
        //         and user-managed callback state. From out perspective
        //         this is thread safe, as long as the caller's code is.
        unsafe impl ::std::marker::Send for $name {}
        unsafe impl ::std::marker::Sync for $name {}

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

        #[allow(unused_mut)]
        unsafe impl $crate::lang::TypeInfo for $name {
            fn type_info() -> $crate::lang::Type {
                use $crate::lang::{TypeInfo, Type, Meta, Docs, Primitive, Parameter, Signature, FnPointer};

                let rval = < $rval as TypeInfo >::type_info();

                let params = vec![
                $(
                    Parameter::new(stringify!($param).to_string(), < $ty as TypeInfo >::type_info()),
                )*
                    Parameter::new("callback_data".to_string(), Type::ReadPointer(Box::new(Type::Primitive(Primitive::Void)))),
                ];

                let mut namespace = ::std::string::String::new();
                $(
                    namespace = ::std::string::String::from($ns);
                )*

                let meta = Meta::with_module_docs(namespace, Docs::new());
                let sig = Signature::new(params, rval);
                let fn_pointer = FnPointer::new_named(sig, stringify!($name).to_string());
                let named_callback = $crate::pattern::callback::NamedCallback::with_meta(fn_pointer, meta);

                Type::Pattern($crate::pattern::TypePattern::NamedCallback(named_callback))
            }
        }
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn callback_default() {
        callback!(MyCallback());
        MyCallback::default();
    }
}
