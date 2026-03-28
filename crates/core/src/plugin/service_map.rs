//! Trait-based mapping of service handles through wrapper types.
//!
//! When a plugin function returns a service handle wrapped in [`ffi::Result`](crate::ffi::Result),
//! [`ffi::Option`](crate::ffi::Option), or nested combinations thereof, the
//! [`ServiceHandleMap`] trait provides a uniform way to replace the raw handle
//! with a concrete service struct — without hand-writing a `match` for every wrapper type.
//!
//! The `plugin!` macro uses this trait in its generated code so that all service-returning
//! functions, regardless of how the handle is wrapped, emit the same one-liner:
//!
//! ```ignore
//! raw.map_service_handle(|h| MyService { handle: h, /* fn ptrs */ })
//! ```
//!
//! Users who define custom wrapper types can implement [`ServiceHandleMap`] to make them
//! transparent to the plugin code generator.

use crate::inventory::{Inventory, TypeId};
use crate::wire::SerializationError;
use crate::lang::types::{Type, TypeInfo, TypeKind, WireIO};

/// An opaque, FFI-safe handle to a service instance.
///
/// `#[repr(transparent)]` over `*const T` so it has identical ABI, but unlike
/// bare `*const T` it is `Send + Sync` — the pointer is never dereferenced on
/// the Rust side and simply shuttles an opaque address to/from the foreign runtime.
#[repr(transparent)]
pub struct ServiceHandle<T>(*const T);

// Manual impls so they don't require T: Copy/Clone/etc.
impl<T> Copy for ServiceHandle<T> {}
impl<T> Clone for ServiceHandle<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> core::fmt::Debug for ServiceHandle<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ServiceHandle").field(&self.0).finish()
    }
}
impl<T> PartialEq for ServiceHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T> Eq for ServiceHandle<T> {}
impl<T> core::hash::Hash for ServiceHandle<T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

unsafe impl<T> Send for ServiceHandle<T> {}
unsafe impl<T> Sync for ServiceHandle<T> {}

impl<T> ServiceHandle<T> {
    /// Wraps a raw pointer in a `ServiceHandle`.
    pub fn from_ptr(ptr: *const T) -> Self {
        Self(ptr)
    }

    /// Returns the underlying raw pointer.
    #[must_use]
    pub fn as_ptr(self) -> *const T {
        self.0
    }
}

unsafe impl<T: TypeInfo> TypeInfo for ServiceHandle<T> {
    const WIRE_SAFE: bool = false;
    const RAW_SAFE: bool = false;
    const ASYNC_SAFE: bool = false;
    const SERVICE_SAFE: bool = false;
    const SERVICE_CTOR_SAFE: bool = false;

    fn id() -> TypeId {
        // Same derived ID as *const T so the inventory sees the same type.
        <*const T as TypeInfo>::id()
    }

    fn kind() -> TypeKind {
        <*const T as TypeInfo>::kind()
    }

    fn ty() -> Type {
        <*const T as TypeInfo>::ty()
    }

    fn register(inventory: &mut impl Inventory) {
        T::register(inventory);
        inventory.register_type(Self::id(), Self::ty());
    }
}

unsafe impl<T: WireIO> WireIO for ServiceHandle<T> {
    fn write(&self, _: &mut impl std::io::Write) -> Result<(), SerializationError> {
        crate::bad_wire!()
    }

    fn read(_: &mut impl std::io::Read) -> Result<Self, SerializationError> {
        crate::bad_wire!()
    }

    fn live_size(&self) -> usize {
        crate::bad_wire!()
    }
}

/// Maps a contained service handle to a concrete service instance.
///
/// Implemented for [`ServiceHandle<S>`] (identity), and recursively for wrapper types
/// like [`ffi::Result`](crate::ffi::Result) and [`ffi::Option`](crate::ffi::Option).
pub trait ServiceHandleMap<S>: Sized {
    /// The type produced when the inner handle is replaced with `T`.
    type Mapped<T>;

    /// Replace the contained handle with the value produced by `f`.
    fn map_service_handle<T>(self, f: impl FnOnce(ServiceHandle<S>) -> T) -> Self::Mapped<T>;
}

// ---------------------------------------------------------------------------
// Identity: ServiceHandle<S> IS the handle
// ---------------------------------------------------------------------------

impl<S> ServiceHandleMap<S> for ServiceHandle<S> {
    type Mapped<T> = T;

    #[inline]
    fn map_service_handle<T>(self, f: impl FnOnce(Self) -> T) -> T {
        f(self)
    }
}

// ---------------------------------------------------------------------------
// ffi::Result — map through the Ok variant
// ---------------------------------------------------------------------------

impl<Inner: ServiceHandleMap<S>, E, S> ServiceHandleMap<S> for crate::pattern::result::Result<Inner, E> {
    type Mapped<T> = crate::pattern::result::Result<Inner::Mapped<T>, E>;

    #[inline]
    fn map_service_handle<T>(self, f: impl FnOnce(ServiceHandle<S>) -> T) -> Self::Mapped<T> {
        match self {
            Self::Ok(s) => crate::pattern::result::Result::Ok(s.map_service_handle(f)),
            Self::Err(e) => crate::pattern::result::Result::Err(e),
            Self::Panic => crate::pattern::result::Result::Panic,
            Self::Null => crate::pattern::result::Result::Null,
        }
    }
}

// ---------------------------------------------------------------------------
// ffi::Option — map through the Some variant
// ---------------------------------------------------------------------------

impl<Inner: ServiceHandleMap<S>, S> ServiceHandleMap<S> for crate::pattern::option::Option<Inner> {
    type Mapped<T> = crate::pattern::option::Option<Inner::Mapped<T>>;

    #[inline]
    fn map_service_handle<T>(self, f: impl FnOnce(ServiceHandle<S>) -> T) -> Self::Mapped<T> {
        match self {
            Self::Some(s) => crate::pattern::option::Option::Some(s.map_service_handle(f)),
            Self::None => crate::pattern::option::Option::None,
        }
    }
}

/// Computes the FFI representation of a type that wraps a service `S`,
/// replacing `S` with [`ServiceHandle<S>`].
///
/// The compiler resolves type aliases *before* trait selection, so this works
/// transparently through aliases such as `type Try<T> = ffi::Result<T, Error>`.
///
/// Only implemented for *wrapper* types ([`ffi::Result`](crate::ffi::Result),
/// [`ffi::Option`](crate::ffi::Option)).  Bare service returns (`-> S`) are
/// handled directly by the `plugin!` macro without this trait.
pub trait ServiceAs<S> {
    /// The FFI-safe representation with `S` replaced by `ServiceHandle<S>`.
    type FFI;
}

impl<S, E> ServiceAs<S> for crate::pattern::result::Result<S, E> {
    type FFI = crate::pattern::result::Result<ServiceHandle<S>, E>;
}

impl<S> ServiceAs<S> for crate::pattern::option::Option<S> {
    type FFI = crate::pattern::option::Option<ServiceHandle<S>>;
}

/// Implemented by plugin-generated service structs to extract the opaque handle.
pub trait PluginService: Sized {
    /// Returns the opaque service handle.
    fn service_handle(&self) -> ServiceHandle<Self>;

    /// Consumes `self` and returns the handle **without** calling `Drop`.
    fn into_service_handle(self) -> ServiceHandle<Self>;
}
