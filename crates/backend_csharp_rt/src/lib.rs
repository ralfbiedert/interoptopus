//! Runtime loaders for Interoptopus plugins.
//!
//! Provides two runtime backends:
//! - [`dynamic`] — Hosts the .NET CLR via `netcorehost` and loads managed assemblies.
//! - [`aot`] — Loads ahead-of-time compiled native libraries via `libloading`.

pub mod aot;
pub mod dynamic;
mod error;
mod shared;

pub use error::DotnetError;
