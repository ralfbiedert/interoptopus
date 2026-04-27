//! Runtime loaders for Interoptopus plugins.
//!
//! Provides two runtime backends:
//! - [`dynamic`] — Hosts the .NET CLR via `netcorehost` and loads managed assemblies.
//! - [`aot`] — Loads ahead-of-time compiled native libraries via `libloading`.
//!

#[cfg(feature = "unstable-rt-aot")]
pub mod aot;
#[cfg(feature = "unstable-rt-dotnet")]
pub mod dynamic;
#[cfg(feature = "unstable-rt-dotnet")]
mod error;
#[cfg(any(feature = "unstable-rt-aot", feature = "unstable-rt-dotnet"))]
mod shared;

#[cfg(feature = "unstable-rt-dotnet")]
pub use error::RuntimeError;
use interoptopus::inventory::Inventory;
use interoptopus::lang::types::TypeInfo;

#[cfg(any(feature = "unstable-rt-aot", feature = "unstable-rt-dotnet"))]
use std::ops::Deref;
#[cfg(any(feature = "unstable-rt-aot", feature = "unstable-rt-dotnet"))]
use std::sync::Arc;

/// A loaded plugin instance, like `Plugin<Foo>`.
#[cfg(any(feature = "unstable-rt-aot", feature = "unstable-rt-dotnet"))]
#[derive(Clone)]
pub struct Plugin<T> {
    inner: Arc<T>,
}

#[cfg(any(feature = "unstable-rt-aot", feature = "unstable-rt-dotnet"))]
impl<T> Plugin<T> {
    pub(crate) fn new(inner: Arc<T>) -> Self {
        Self { inner }
    }
}

#[cfg(any(feature = "unstable-rt-aot", feature = "unstable-rt-dotnet"))]
impl<T> Deref for Plugin<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}
