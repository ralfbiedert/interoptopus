//! Tera-based template rendering with embedded asset support.
//!
//! Templates are packed into a tar archive at build time (via [`pack_assets`] in
//! `build.rs`), embedded in the binary, and loaded at runtime into a
//! [`TemplateEngine`]. The [`render!`](crate::render) macro provides a convenient
//! shorthand for rendering with key-value context pairs.

mod assets;
mod engine;
mod macros;

pub use assets::{Assets, pack_assets};
pub use engine::TemplateEngine;
pub use tera::{Context, Value};
