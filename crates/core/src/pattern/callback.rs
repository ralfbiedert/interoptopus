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


        impl $crate::lang::types::TypeInfo for $name {
            const WIRE_SAFE: bool = <$rval>::WIRE_SAFE $(&& <$ty>::WIRE_SAFE)*;
            const RAW_SAFE: bool = <$rval>::RAW_SAFE $(&& <$ty>::RAW_SAFE)*;

            fn id() -> $crate::inventory::TypeId {
                $crate::type_id!($name)
            }

            fn kind() -> $crate::lang::types::TypeKind {
                let sig = $crate::lang::function::Signature {
                    arguments: vec![
                        $($crate::lang::function::Argument::new(stringify!($param), <$ty>::id()),)*
                    ],
                    rval: <$rval>::id(),
                };
                $crate::lang::types::TypeKind::TypePattern($crate::lang::types::TypePattern::NamedCallback(sig))
            }

            fn ty() -> $crate::lang::types::Type {
                let r = <$rval>::ty();
                $(let $param = <$ty>::ty();)*

                let emissision = [
                    r.emission.clone(),
                    $($param.emission.clone(),)*
                ];

                let sig = $crate::lang::function::Signature {
                    arguments: vec![
                        $($crate::lang::function::Argument::new(stringify!($param), <$ty>::id()),)*
                    ],
                    rval: <$rval>::id(),
                };

                $crate::lang::types::Type {
                    emission: $crate::lang::meta::common_or_module_emission(&emissision),
                    docs: $crate::lang::meta::Docs::empty(),
                    visibility: $crate::lang::meta::Visibility::Public,
                    name: stringify!($name).to_string(),
                    kind: $crate::lang::types::TypeKind::TypePattern($crate::lang::types::TypePattern::NamedCallback(sig)),
                }
            }

            fn register(inventory: &mut $crate::inventory::Inventory) {
                // Register contained types
                <$rval>::register(inventory);
                $(<$ty>::register(inventory);)*
                <*const ::std::ffi::c_void>::register(inventory);
                <extern "C" fn($($ty,)* *const ::std::ffi::c_void) -> $rval>::register(inventory);

                inventory.register_type(Self::id(), Self::ty());
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
