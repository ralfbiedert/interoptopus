#![doc = include_str!("../README.md")]
#![allow(unused)] // TODO: For now, since lots of bare bones structs
#![allow(unexpected_cfgs)]
#![allow(clippy::too_many_arguments)] // Pass functions take many context parameters by design
#![allow(clippy::type_complexity)] // Complex types in pass pipeline are expected

pub mod dispatch;
pub mod extensions;
pub mod lang;
pub mod output;
pub mod pass;
pub mod template;

pub mod config;
mod error;
mod macros;
mod pipeline;
#[cfg(any(feature = "unstable-plugins", docsrs))]
pub mod plugin;

pub use error::Error;
#[cfg(any(feature = "unstable-plugins", docsrs))]
pub use pipeline::{DotnetLibrary, DotnetLibraryBuilder};
pub use pipeline::{RustLibrary, RustLibraryBuilder};
