//! Trait-based mapping of service handles through wrapper types.
//!
//! When a plugin function returns a service handle wrapped in [`ffi::Result`](crate::ffi::Result),
//! [`ffi::Option`](crate::ffi::Option), or nested combinations thereof, the
//! [`ServiceHandleMap`] trait provides a uniform way to replace the raw `isize` handle
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

/// Maps a contained `isize` service handle to a concrete service instance.
///
/// Implemented for `isize` (identity), and recursively for wrapper types
/// like [`ffi::Result`](crate::ffi::Result) and [`ffi::Option`](crate::ffi::Option).
pub trait ServiceHandleMap: Sized {
    /// The type produced when the inner `isize` is replaced with `T`.
    type Mapped<T>;

    /// Replace the contained handle with the value produced by `f`.
    fn map_service_handle<T>(self, f: impl FnOnce(isize) -> T) -> Self::Mapped<T>;
}

// ---------------------------------------------------------------------------
// Identity: isize IS the handle
// ---------------------------------------------------------------------------

impl ServiceHandleMap for isize {
    type Mapped<T> = T;

    #[inline]
    fn map_service_handle<T>(self, f: impl FnOnce(isize) -> T) -> T {
        f(self)
    }
}

// ---------------------------------------------------------------------------
// ffi::Result — map through the Ok variant
// ---------------------------------------------------------------------------

impl<S: ServiceHandleMap, E> ServiceHandleMap for crate::pattern::result::Result<S, E> {
    type Mapped<T> = crate::pattern::result::Result<S::Mapped<T>, E>;

    #[inline]
    fn map_service_handle<T>(self, f: impl FnOnce(isize) -> T) -> Self::Mapped<T> {
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

impl<S: ServiceHandleMap> ServiceHandleMap for crate::pattern::option::Option<S> {
    type Mapped<T> = crate::pattern::option::Option<S::Mapped<T>>;

    #[inline]
    fn map_service_handle<T>(self, f: impl FnOnce(isize) -> T) -> Self::Mapped<T> {
        match self {
            Self::Some(s) => crate::pattern::option::Option::Some(s.map_service_handle(f)),
            Self::None => crate::pattern::option::Option::None,
        }
    }
}

/// Implemented by plugin-generated service structs to extract the opaque handle.
pub trait PluginService {
    /// Returns the opaque service handle.
    fn service_handle(&self) -> isize;

    /// Consumes `self` and returns the handle **without** calling `Drop`.
    fn into_service_handle(self) -> isize;
}
