//! Like a regular [`Result`](std::result::Result), but FFI safe.
//!
//!
//! # Examples
//!
//! Functions returning a [`Result`] might receive special treatment in backends supporting
//! exception handling. For example, a [`service`](crate::pattern::service) method defined
//! as:
//!
//! ```
//! # use interoptopus::Error;
//! #
//! pub fn my_method() -> Result<(), Error> {
//!     Ok(())
//! }
//! ```
//!
//! might receive a binding helper equivalent to:
//!
//! ```csharp
//! public void MyMethod()
//! {
//!     var rval = Interop.simple_service_my_method(_context);
//!     if (rval != FFIError.Ok)
//!     {
//!         throw new Exception($"Something went wrong {rval}");
//!     }
//! }
//! ```

use crate::inventory::{Inventory, TypeId};
use crate::lang::meta::{Emission, Visibility};
use crate::lang::types::{SerializationError, Type, TypeInfo, TypeKind, TypePattern};
use std::any::Any;
use std::fmt::Debug;
use std::io::{Read, Write};
use std::panic::{AssertUnwindSafe, catch_unwind};

/// Extracts a string message from a panic unwind.
pub fn get_panic_message(pan: &(dyn Any + Send)) -> &str {
    match pan.downcast_ref::<&'static str>() {
        Some(s) => s,
        None => match pan.downcast_ref::<String>() {
            Some(s) => s,
            None => "Any { .. }",
        },
    }
}

#[repr(u32)]
#[derive(Debug)]
pub enum Result<T, E> {
    Ok(T),
    Err(E),
    /// Internal variant used when a panic occurred.
    Panic,
    /// Internal variant used when null was passed where it shouldn't.
    Null,
}

// impl<T: ServiceInfo, E> Result<T, E> {
//     // pub const ASSERT_CTOR_RVAL: bool = true;
// }

impl<T, E> ResultAs for Result<T, E> {
    type AsT<X> = Result<X, E>;
}

impl<T, E> Result<T, E>
where
    T: TypeInfo,
    E: TypeInfo,
{
    /// Returns `true` if the result is `Ok`.
    #[must_use]
    pub fn is_ok(&self) -> bool {
        match self {
            Self::Ok(_) => true,
            Self::Err(_) => false,
            Self::Panic => false,
            Self::Null => false,
        }
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

impl<T: TypeInfo, E: TypeInfo> TypeInfo for Result<T, E> {
    const WIRE_SAFE: bool = T::WIRE_SAFE && E::WIRE_SAFE;
    const RAW_SAFE: bool = T::RAW_SAFE && E::RAW_SAFE;
    const ASYNC_SAFE: bool = T::ASYNC_SAFE && E::ASYNC_SAFE;
    const SERVICE_SAFE: bool = false;
    const SERVICE_CTOR_SAFE: bool = T::SERVICE_SAFE;

    fn id() -> TypeId {
        TypeId::new(0x9BCBD2325F73A8CBDAE991B5BB8EB6FC).derive_id(T::id())
    }

    fn kind() -> TypeKind {
        TypeKind::TypePattern(TypePattern::Result(T::id(), E::id()))
    }

    fn ty() -> Type {
        let t = T::ty();
        let e = E::ty();
        Type {
            emission: Emission::Common,
            docs: crate::lang::meta::Docs::empty(),
            visibility: Visibility::Public,
            name: format!("Result<{}, {}>", t.name, e.name),
            kind: Self::kind(),
        }
    }

    fn register(inventory: &mut Inventory) {
        // Ensure base types are registered.
        T::register(inventory);
        E::register(inventory);
        inventory.register_type(Self::id(), Self::ty());
    }

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
