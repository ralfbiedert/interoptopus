//! FFI-safe [`Result<T, E>`](std::result::Result) with backend-specific error handling.
//!
//! [`Result<T, E>`] is a `repr(C)` enum that carries either a success
//! value or an error variant across the FFI boundary. Backends that
//! support the pattern translate error variants into idiomatic
//! exceptions — for example, a C# backend converts a failed result into
//! a CLR exception automatically.
//!
//! # Example
//!
//! ```
//! use interoptopus::ffi;
//!
//! #[ffi]
//! pub enum MyError { BadInput }
//!
//! #[ffi]
//! pub fn parse(x: u32) -> ffi::Result<u32, MyError> {
//!     if x == 0 { ffi::Err(MyError::BadInput) }
//!     else { ffi::Ok(x * 2) }
//! }
//! ```

use crate::inventory::{Inventory, TypeId};
use crate::lang::meta::{Docs, Visibility, common_or_module_emission};
use crate::lang::types::{Type, TypeInfo, TypeKind, TypePattern, WireIO, type_id_ptr};
use crate::pattern::asynk::CancelValue;
use crate::wire::SerializationError;
use std::fmt::Debug;
use std::io::{Read, Write};
use std::panic::{AssertUnwindSafe, catch_unwind};

/// FFI-safe result type.
///
/// See the [module documentation](crate::pattern::result) for more details and examples.
#[repr(u32)]
#[derive(Debug, Clone)]
#[must_use]
pub enum Result<T, E> {
    Ok(T),
    Err(E),
    /// Internal variant used when a panic occurred.
    Panic,
    /// Internal variant used when null was passed where it shouldn't.
    Null,
}

impl<T, E> ResultAs for Result<T, E> {
    type AsT<X> = Result<X, E>;
}

impl<T, E> CancelValue for Result<T, E> {
    fn cancel_value() -> Self {
        Self::Panic
    }
}

impl<T, E> Result<T, E> {
    /// Returns `true` if the result is `Ok`.
    #[must_use]
    pub fn is_ok(&self) -> bool {
        matches!(self, Self::Ok(_))
    }

    /// Returns the `Ok` variant if it exists, otherwise panics.
    pub fn unwrap(self) -> T {
        if let Self::Ok(t) = self {
            t
        } else {
            panic!("Called `unwrap` on an `FFIResult` that is not `Ok`.")
        }
    }

    /// Returns the `Err` variant if it exists, otherwise panics.
    pub fn unwrap_err(self) -> E {
        if let Self::Err(err) = self {
            err
        } else {
            panic!("Called `unwrap_err` on an `FFIResult` that is not `Err`.")
        }
    }
}

impl<T, E> From<std::result::Result<T, E>> for Result<T, E>
where
    T: TypeInfo,
    E: TypeInfo,
{
    fn from(x: std::result::Result<T, E>) -> Self {
        match x {
            Ok(t) => Self::Ok(t),
            Err(err) => Self::Err(err),
        }
    }
}

unsafe impl<T: TypeInfo, E: TypeInfo> TypeInfo for Result<T, E> {
    const WIRE_SAFE: bool = T::WIRE_SAFE && E::WIRE_SAFE;
    const RAW_SAFE: bool = T::RAW_SAFE && E::RAW_SAFE;
    const ASYNC_SAFE: bool = T::ASYNC_SAFE && E::ASYNC_SAFE;
    const SERVICE_SAFE: bool = false;
    const SERVICE_CTOR_SAFE: bool = T::SERVICE_SAFE;

    fn id() -> TypeId {
        // When T is a service type, the FFI representation uses *const T (a pointer)
        // rather than T directly, because services are always passed as opaque pointers
        // across the FFI boundary. This makes Result<ServiceType, E> register with the
        // same identity as Result<*const ServiceType, E>, matching the constructor pattern.
        let is_service = matches!(T::kind(), TypeKind::Service);
        let ok_id = if is_service { type_id_ptr(T::id()) } else { T::id() };
        TypeId::new(0x9BCBD2325F73A8CBDAE991B5BB8EB6FC).derive_id(ok_id).derive_id(E::id())
    }

    fn kind() -> TypeKind {
        let is_service = matches!(T::kind(), TypeKind::Service);
        let ok_id = if is_service { type_id_ptr(T::id()) } else { T::id() };
        TypeKind::TypePattern(TypePattern::Result(ok_id, E::id()))
    }

    fn ty() -> Type {
        let t = T::ty();
        let e = E::ty();
        Type {
            emission: common_or_module_emission(&[t.emission, e.emission]),
            docs: Docs::from_line("Rust-like `Result` type usable over FFI."),
            visibility: Visibility::Public,
            name: format!("Result<{}, {}>", t.name, e.name),
            kind: Self::kind(),
        }
    }

    fn register(inventory: &mut impl Inventory) {
        // Ensure base types are registered.
        T::register(inventory);
        E::register(inventory);
        // When T is a service, also register *const T so the pointer type exists
        // in the inventory for the Ok variant.
        if matches!(T::kind(), TypeKind::Service) {
            <*const T as TypeInfo>::register(inventory);
        }
        inventory.register_type(Self::id(), Self::ty());
    }
}

unsafe impl<T: WireIO, E: WireIO> WireIO for Result<T, E> {
    fn write(&self, _: &mut impl Write) -> std::result::Result<(), SerializationError> {
        todo!()
    }

    fn read(_: &mut impl Read) -> std::result::Result<Self, SerializationError> {
        todo!()
    }

    fn live_size(&self) -> usize {
        todo!()
    }
}

/// At some point we want to get rid of these once `Try` ([try_trait_v2](https://github.com/rust-lang/rust/issues/84277)) stabilizes.
pub fn result_to_ffi<T: TypeInfo, E: TypeInfo>(f: impl FnOnce() -> std::result::Result<T, E>) -> Result<T, E> {
    f().into()
}

/// At some point we want to get rid of these once `Try` ([try_trait_v2](https://github.com/rust-lang/rust/issues/84277)) stabilizes.
pub async fn result_to_ffi_async<T: TypeInfo, E: TypeInfo>(f: impl std::ops::AsyncFnOnce() -> std::result::Result<T, E>) -> Result<T, E> {
    f().await.into()
}

/// Converts a panic to a [`Result::Panic`].
pub fn panic_to_result<T: TypeInfo, E: TypeInfo>(f: impl FnOnce() -> Result<T, E>) -> Result<T, E> {
    catch_unwind(AssertUnwindSafe(f)).unwrap_or_else(|_| Result::Panic)
}

/// Internal helper trait converting `Result<T, E>` into `Result<*const T, E>`.
///
/// This type is mainly used from our proc macros that need to name the related
/// type, without having access to reflection.
#[doc(hidden)]
pub trait ResultAs {
    type AsT<T>;
}
