//! Reverse-interop plugin support.
//!
//! This module contains traits and constants used by the `plugin!` macro
//! to load foreign plugins (e.g., .NET DLLs) and call their functions
//! from Rust.

pub mod service_map;
pub mod trampoline;
