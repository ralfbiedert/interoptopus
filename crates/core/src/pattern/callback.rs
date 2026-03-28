//! Named delegates in higher languages.
//!
//! # Example
//!
//! If you want to accept user-provided callbacks or "delegates":
//!
//!```
//! use interoptopus::{ffi, callback};
//! use interoptopus::pattern::slice::Slice;
//!
//! callback!(CallbackSlice(x: Slice<'_, u8>) -> u8);
//!
//! #[ffi]
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
//! implement [`TypeInfo`](crate::lang::types::TypeInfo) for _all_ types you may want to use;
//! [`FFISlice`](crate::pattern::slice::Slice) here being one of them.
//! To fix this, you can replace `pub type CallbackSlice = ...` with a `callback!` call
//! which should generate a helper type that works.
//!

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
/// let callback = MyCallback::from_fn(|| 42);
/// assert_eq!(42, callback.call());
/// ```
#[macro_export]
macro_rules! callback {
    ($name:ident($($param:ident: $ty:ty),*)) => {
        callback!($name($($param: $ty),*) -> ());
    };

    ($name:ident($($param:ident: $ty:ty),*) -> $rval:ty $(, namespace = $ns:expr)?) => {
        #[derive(Default)]
        #[repr(C)]
        pub struct $name {
            /// The function pointer to invoke. `None` means the callback is absent.
            pub callback: ::std::option::Option<extern "C" fn($($ty,)* *const ::std::ffi::c_void) -> $rval>,
            /// Opaque context pointer passed as the last argument on every call.
            pub data: *const ::std::ffi::c_void,
            /// Optional destructor for `data`. Set by [`from_closure`](Self::from_closure);
            /// `None` for plain function-pointer callbacks. The foreign caller (e.g. C#
            /// `Dispose()`) must invoke this exactly once when done with the callback.
            pub destructor: ::std::option::Option<unsafe extern "C" fn(*const ::std::ffi::c_void)>,
        }

        // Safety: This is a transparent wrapper around a function pointer
        //         and user-managed callback state. From our perspective
        //         this is thread safe, as long as the caller's code is.
        unsafe impl ::std::marker::Send for $name {}
        unsafe impl ::std::marker::Sync for $name {}

        impl ::std::ops::Drop for $name {
            fn drop(&mut self) {
                if let Some(dtor) = self.destructor {
                    // Safety: destructor is only set by from_closure, which boxes the closure
                    // and stores the raw pointer in self.data. Drop runs exactly once.
                    unsafe { dtor(self.data) };
                }
            }
        }

        impl $name {
            /// Creates a callback from a Rust closure.
            ///
            /// The closure is heap-allocated and owned by this value. When the value is dropped
            /// on the Rust side the allocation is freed automatically. When ownership is moved
            /// across an FFI boundary (e.g. to C#), the foreign caller is responsible for
            /// invoking the destructor stored in the third field (e.g. via `Dispose()`).
            pub fn from_fn<F>(f: F) -> Self
            where
                F: Fn($($ty),*) -> $rval + Send + Sync + 'static,
            {
                extern "C" fn trampoline<F: Fn($($ty),*) -> $rval>(
                    $($param: $ty,)*
                    ctx: *const ::std::ffi::c_void,
                ) -> $rval {
                    let f = unsafe { &*(ctx as *const F) };
                    f($($param,)*)
                }

                unsafe extern "C" fn destructor<F>(ctx: *const ::std::ffi::c_void) {
                    drop(unsafe { ::std::boxed::Box::from_raw(ctx as *mut F) });
                }

                let ptr = ::std::boxed::Box::into_raw(::std::boxed::Box::new(f)) as *const ::std::ffi::c_void;
                Self { callback: Some(trampoline::<F>), data: ptr, destructor: Some(destructor::<F>) }
            }

            /// Will call function if it exists, panic otherwise.
            pub fn call(&self, $($param: $ty),*) -> $rval {
                self.callback.expect("Assumed function would exist but it didn't.")($($param,)* self.data)
            }

            /// Will call function only if it exists
            pub fn call_if_some(&self, $($param: $ty,)*) -> ::std::option::Option<$rval> {
                match self.callback {
                    Some(c) => Some(c($($param,)* self.data)),
                    None => None
                }
            }
        }

        impl From<for<> extern "C" fn($($ty,)* *const ::std::ffi::c_void) -> $rval> for $name {
            fn from(x: extern "C" fn($($ty,)* *const ::std::ffi::c_void) -> $rval) -> Self {
                Self { callback: Some(x), data: ::std::ptr::null(), destructor: None }
            }
        }

        impl From<$name> for ::std::option::Option<extern "C" fn($($ty,)* *const ::std::ffi::c_void) -> $rval> {
            fn from(x: $name) -> Self {
                x.callback
            }
        }


        unsafe impl $crate::lang::types::TypeInfo for $name {
            const WIRE_SAFE: bool = <$rval>::WIRE_SAFE $(&& <$ty>::WIRE_SAFE)*;
            const RAW_SAFE: bool = <$rval>::RAW_SAFE $(&& <$ty>::RAW_SAFE)*;
            const ASYNC_SAFE: bool = <$rval>::ASYNC_SAFE $(&& <$ty>::ASYNC_SAFE)*;
            const SERVICE_SAFE: bool = false;
            const SERVICE_CTOR_SAFE: bool = false;

            fn id() -> $crate::inventory::TypeId {
                $crate::inventory::TypeId::from_id($crate::id!($name))
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

            fn register(inventory: &mut impl $crate::inventory::Inventory) {
                // Register contained types
                <$rval>::register(inventory);
                $(<$ty>::register(inventory);)*
                <*const ::std::ffi::c_void>::register(inventory);
                // This gives compile errors on struct with `Slice<'a, _>` and similar, which
                // apparently is the old `fn f()` vs. `for<> fn f()` (compiler) bug?
                // We should get away 'forgetting' to register this particular type, because
                // no one will be naming it (in fact no one can be naming it due to the same issues
                // we're having), and backends will just see a `NamedCallback` pattern and should be
                // fine.
                // <extern "C" fn($($ty,)* *const ::std::ffi::c_void) -> $rval>::register(inventory);
                inventory.register_type(Self::id(), Self::ty());
            }

        }

        unsafe impl $crate::lang::types::WireIO for $name {
            fn write(&self, _: &mut impl ::std::io::Write) -> Result<(), $crate::wire::SerializationError> {
                $crate::bad_wire!()
            }

            fn read(_: &mut impl ::std::io::Read) -> Result<Self, $crate::wire::SerializationError> {
                $crate::bad_wire!()
            }

            fn live_size(&self) -> usize {
                $crate::bad_wire!()
            }

        }
    };
}
