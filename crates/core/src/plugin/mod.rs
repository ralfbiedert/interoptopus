//! Reverse-interop plugin support.
//!
//! This module contains traits and constants used by the `plugin!` macro
//! to load foreign plugins (e.g., .NET DLLs) and call their functions
//! from Rust.

mod service_map;
pub mod trampoline;

#[doc(hidden)]
pub use service_map::{PluginService, ServiceAs, ServiceHandle, ServiceHandleMap};
