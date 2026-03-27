//! Runtime loaders for Interoptopus plugins.
//!
//! Provides two runtime backends:
//! - [`dynamic`] — Hosts the .NET CLR via `netcorehost` and loads managed assemblies.
//! - [`aot`] — Loads ahead-of-time compiled native libraries via `libloading`.
//!

#[cfg(feature = "rt-aot")]
pub mod aot;
#[cfg(feature = "rt-dotnet")]
pub mod dynamic;
#[cfg(feature = "rt-dotnet")]
mod error;
#[cfg(any(feature = "rt-aot", feature = "rt-dotnet"))]
mod shared;

#[cfg(feature = "rt-dotnet")]
pub use error::RuntimeError;
use interoptopus::inventory::Inventory;
use interoptopus::lang::types::TypeInfo;
