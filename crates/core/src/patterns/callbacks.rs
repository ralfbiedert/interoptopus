//! Useful for short- and long-lived callbacks.
//!
//! Callbacks are supported in two flavors:
//!
//! - **Immediate callbacks** for use only within the current function.    
//! - **Retained callbacks** that are stored inside a service and invoked later.
//!  
//! Under the hood both callback types are just (named) function pointers or delegates.
//!
//! However, some backends
//! might produce different code, depending on the callback's intended use. Immediate callbacks
//! are likely a bit faster and more memory efficient, while retained callbacks might instruct
//! a hosting runtime to generate and store auxiliary wrapper data.
//!
//! ⚠️ You are strongly advised to only use callbacks according to their designation. If you
//! stored immediate callbacks past their method call you might get undefined behavior; if
//! you used retained callbacks in an "immediate hot loop" you might run out of memory.
//!
//!
//!
//! # Example
//!
//! If you want to accept a user-provided callback for immediate use:
//!
//!```
//! use interoptopus::{ffi_function, callback_immediate};
//!
//! callback_immediate!(Callback(x: u8));
//!
//! #[ffi_function]
//! pub fn my_function(callback: Callback) {
//!     callback.call(123);
//! }
//! ```
//! Backends supporting this pattern might generate the equivalent to the following pseudo-code:
//!
//! ```csharp
//! public delegate void Callback(byte x0);
//! void my_function(Callback callback);
//! ```
//!
//! Backends not supporting this pattern, and C FFI, will see the equivalent of the following C code:
//! ```c
//! typedef void (*fptr_fn_u8)(my_library_u8 x0);
//! void my_function(fptr_fn_u8 callback);
//! ```
//!
//!
//! # Code Generation
//!
//! The macro [**`callback`**](crate::callback_immediate) enables two use cases:
//!
//! - On the **Rust** side it will generate a new function-pointer type with better compatibility
//!   with respect to lifetimes in signatures, and accepting an unlimited number of args.
//! - On the **FFI side** a _properly named_ callback (delegate, function pointer ...) type can be
//!   produced (e.g., `MyCallback`), instead of one where it's name is just a generic concatenation
//!   of all used parameters (e.g., `InteropDelegate_fn_i32_i32`).
//!
//! # Immediate Callback Lifetimes ⚠️
//!
//! As mentioned, immediate callbacks passed to functions or methods are assumed to live
//! only for the duration of that call, while retained callbacks can be stored.
//! Immediate callbacks come with a "hidden" lifetime parameter that exists to prevent you
//! from accidentally storing the function. In normal use this parameter
//! can be elided like so:
//!
//!```
//! use interoptopus::{ffi_function, callback_immediate};
//!
//! callback_immediate!(Callback());
//!
//! #[ffi_function]
//! pub fn my_function(callback: Callback) { }
//!```
//! However, if your function takes other parameters by reference, you might have
//! to specify it explicitly:
//!
//!```
//! # use interoptopus::{ffi_function, callback_immediate};
//! # callback_immediate!(Callback());
//! #[ffi_function]
//! pub fn my_function<'a>(x: &'a u8, callback: Callback<'a>) { }
//!```
//!
//! Likewise, returning a Rust function can be achieved by specifying a `'static` lifetime:
//!
//!```
//! # use interoptopus::{ffi_function, callback_immediate};
//! # callback_immediate!(Callback());
//! #[ffi_function]
//! pub fn my_function() -> Callback<'static> { todo!() }
//!```
//!
//! With that said, nothing would stop you from accepting an immediate callback with a `'static`
//! lifetime, and subsequently storing it inside your Rust code. The result of that would be
//! "unspecified behavior". In some backends, with some function pointers, this might work, while
//! in others you could end up with undefined behavior in case you invoked that callback some time
//! later.
//!
//!```ignore
//! # use interoptopus::{ffi_function, callback_immediate};
//! # callback_immediate!(Callback());
//! #[ffi_function]
//! pub fn my_function(callback: Callback<'static>) { }
//!```
//!
//! # How to return callbacks from functions
//!
//! Due to another Rust limitation this won't work, despite the `From<>` conversion
//! "being implemented".
//!
//! ```rust,ignore
//! use interoptopus::{ffi_function, callback_immediate};
//!
//! callback_immediate!(SumFunction(x: i32, y: i32) -> i32);
//!
//! #[ffi_function]
//! #[no_mangle]
//! pub fn return_sum_function() -> SumFunction<'static> {
//!     my_sum_function.into() // Compile error, mismatch between `function item type` and `function pointer type`
//! }
//!
//! extern "C" fn my_sum_function(x: i32, y: i32) -> i32 { x + y }
//! ```
//!
//! Instead, you will have to return function pointers like so:
//!
//!```rust
//! # use interoptopus::{ffi_function, callback_immediate};
//! # callback_immediate!(SumFunction(x: i32, y: i32) -> i32);
//! # extern "C" fn my_sum_function(x: i32, y: i32) -> i32 { x + y }
//! #
//! #[ffi_function]
//! pub fn return_sum_function() -> SumFunction<'static> {
//!     SumFunction::new(my_sum_function)
//! }
//! ```

use crate::lang::c::{FnPointerType, Meta};

/// Internal helper naming a generated callback type wrapper.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NamedCallback {
    fnpointer: FnPointerType,
    meta: Meta,
}

impl NamedCallback {
    /// Creates a new named callback.
    pub fn new(callback: FnPointerType) -> Self {
        Self::with_meta(callback, Meta::new())
    }

    /// Creates a new named callback with the given meta.
    pub fn with_meta(callback: FnPointerType, meta: Meta) -> Self {
        if callback.name().is_none() {
            panic!("The pointer provided to a named callback must have a name.")
        }
        Self { fnpointer: callback, meta }
    }

    /// Gets the type name of this callback.
    pub fn name(&self) -> &str {
        self.fnpointer.name().unwrap()
    }

    /// Gets the type's meta.
    pub fn meta(&self) -> &Meta {
        &self.meta
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
/// use interoptopus::callback_immediate;
/// use interoptopus::patterns::slice::FFISlice;
///
/// callback_immediate!(MyCallback(slice: FFISlice<u8>) -> u8);
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
/// use interoptopus::callback_immediate;
///
/// callback_immediate!(MyCallback() -> u8);
///
/// extern "C" fn my_rust_callback() -> u8 {
///     42
/// }
///
/// let callback = MyCallback::new(my_rust_callback);
/// assert_eq!(42, callback.call());
/// ```
#[macro_export]
macro_rules! callback_retained {
    ($name:ident($($param:ident: $ty:ty),*)) => {
        callback_retained!($name($($param: $ty),*) -> ());
    };

    ($name:ident($($param:ident: $ty:ty),*) -> $rval:ty $(, namespace = $ns:expr)?) => {
        #[derive(Default, Clone, Copy)]
        #[repr(transparent)]
        pub struct $name(Option<extern "C" fn($($ty),*) -> $rval>);

        impl $name {
            /// Creates a new instance of the callback using `extern "C" fn`
            pub fn new(func: extern "C" fn($($ty),*) -> $rval) -> Self {
                Self(Some(func))
            }

            /// Will call function if it exists, panic otherwise.
            pub fn call(&self, $($param: $ty),*) -> $rval {
                self.0.expect("Assumed function would exist but it didn't.")($($param),*)
            }

            /// Will call function only if it exists
            pub fn call_if_some(&self, $($param: $ty),*) -> Option<$rval> {
                match self.0 {
                    Some(c) => Some(c($($param),*)),
                    None => None
                }
            }
        }

        impl From<for<> extern "C" fn($($ty),*) -> $rval> for $name {
            fn from(x: extern "C" fn($($ty),*) -> $rval) -> Self {
                Self(Some(x))
            }
        }

        impl From<$name> for Option<extern "C" fn($($ty),*) -> $rval> {
            fn from(x: $name) -> Self {
                x.0
            }
        }

        unsafe impl ::interoptopus::lang::rust::CTypeInfo for $name {
            fn type_info() -> ::interoptopus::lang::c::CType {
                use ::interoptopus::lang::rust::CTypeInfo;
                use ::interoptopus::lang::c::{Meta, Documentation};

                let rval = < $rval as CTypeInfo >::type_info();

                let params = vec![
                $(
                    interoptopus::lang::c::Parameter::new(stringify!($param).to_string(), < $ty as CTypeInfo >::type_info()),
                )*
                ];

                let mut namespace = String::new();
                $(
                    namespace = String::from($ns);
                )*

                let meta = Meta::with_namespace_documentation(namespace, Documentation::new());
                let sig = ::interoptopus::lang::c::FunctionSignature::new(params, rval);
                let fn_pointer = ::interoptopus::lang::c::FnPointerType::new_named(sig, stringify!($name).to_string());
                let named_callback = ::interoptopus::patterns::callbacks::NamedCallback::with_meta(fn_pointer, meta);

                ::interoptopus::lang::c::CType::Pattern(::interoptopus::patterns::TypePattern::RetainedCallback(named_callback))
            }
        }
    };
}

#[macro_export]
macro_rules! callback_immediate {
    ($name:ident($($param:ident: $ty:ty),*)) => {
        callback_immediate!($name($($param: $ty),*) -> ());
    };

    ($name:ident($($param:ident: $ty:ty),*) -> $rval:ty $(, namespace = $ns:expr)?) => {
        #[derive(Default, Clone, Copy)]
        #[repr(transparent)]
        pub struct $name<'a> {
            ptr: Option<extern "C" fn($($ty),*) -> $rval>,
            lt: ::std::marker::PhantomData<&'a ()> // TODO: Fix this w.r.t lt subtyping
        }

        impl<'a> $name<'a> {
            /// Creates a new instance of the callback using `extern "C" fn`
            pub fn new(func: extern "C" fn($($ty),*) -> $rval) -> Self {
                Self {
                    ptr: Some(func),
                    lt: ::std::marker::PhantomData::default()
                }
            }

            /// Will call function if it exists, panic otherwise.
            pub fn call(&self, $($param: $ty),*) -> $rval {
                self.ptr.expect("Assumed function would exist but it didn't.")($($param),*)
            }

            /// Will call function only if it exists
            pub fn call_if_some(&self, $($param: $ty),*) -> Option<$rval> {
                match self.ptr {
                    Some(c) => Some(c($($param),*)),
                    None => None
                }
            }
        }

        impl<'a> From<for<> extern "C" fn($($ty),*) -> $rval> for $name<'a> {
            fn from(x: extern "C" fn($($ty),*) -> $rval) -> Self {
                Self {
                    ptr: Some(x),
                    lt: ::std::marker::PhantomData::default()
                }
            }
        }

        impl<'a> From<$name<'a>> for Option<extern "C" fn($($ty),*) -> $rval> {
            fn from(x: $name) -> Self {
                x.ptr
            }
        }

        unsafe impl<'a> ::interoptopus::lang::rust::CTypeInfo for $name<'a> {
            fn type_info() -> ::interoptopus::lang::c::CType {
                use ::interoptopus::lang::rust::CTypeInfo;
                use ::interoptopus::lang::c::{Meta, Documentation};

                let rval = < $rval as CTypeInfo >::type_info();

                let params = vec![
                $(
                    interoptopus::lang::c::Parameter::new(stringify!($param).to_string(), < $ty as CTypeInfo >::type_info()),
                )*
                ];

                let mut namespace = String::new();
                $(
                    namespace = String::from($ns);
                )*

                let meta = Meta::with_namespace_documentation(namespace, Documentation::new());
                let sig = interoptopus::lang::c::FunctionSignature::new(params, rval);
                let fn_pointer = interoptopus::lang::c::FnPointerType::new_named(sig, stringify!($name).to_string());
                let named_callback = interoptopus::patterns::callbacks::NamedCallback::with_meta(fn_pointer, meta);

                interoptopus::lang::c::CType::Pattern(interoptopus::patterns::TypePattern::InstantCallback(named_callback))
            }
        }
    };
}
