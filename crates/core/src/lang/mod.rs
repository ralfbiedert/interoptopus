//! Data model describing the items of an FFI boundary.
//!
//! These types form the intermediate representation that sits between Rust source
//! code (annotated with `#[ffi]`) and the generated target-language bindings. Most
//! users never interact with them directly — they are populated by the proc macros
//! and consumed by backends.
pub mod constant;
pub mod function;
pub mod meta;
#[cfg(feature = "unstable-plugins")]
pub mod plugin;
pub mod service;
pub mod types;
