#![doc = include_str!("../README.md")]
#![allow(unused)] // TODO: For now, since lots of bare bones structs
#![allow(unexpected_cfgs)]
#![allow(clippy::too_many_arguments)] // Pass functions take many context parameters by design
#![allow(clippy::type_complexity)] // Complex types in pass pipeline are expected

pub mod dispatch;
pub mod lang;
pub mod output;
pub mod pass;
pub mod plugin;
pub mod template;

mod error;
mod macros;
mod pipeline;

pub use error::Error;
pub use pipeline::{RustLibrary, RustLibraryConfig};
