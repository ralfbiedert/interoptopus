#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![allow(unused)] // TODO: For now, since lots of bare bones structs
#![allow(unexpected_cfgs)]
#![allow(clippy::too_many_arguments)] // Pass functions take many context parameters by design
#![allow(clippy::type_complexity)] // Complex types in pass pipeline are expected

pub mod dispatch;
#[cfg(any(feature = "unstable-extensions", docsrs))]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable-extensions")))]
pub mod extensions;
#[cfg(not(any(feature = "unstable-extensions", docsrs)))]
pub(crate) mod extensions;
pub mod lang;
pub mod output;
pub mod pass;
pub mod template;

pub mod config;
mod error;
mod macros;
mod pipeline;

pub use error::Error;

pub mod pattern;
#[cfg(any(feature = "unstable-rt-aot", feature = "unstable-rt-dotnet", docsrs))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "unstable-rt-aot", feature = "unstable-rt-dotnet"))))]
pub mod rt;

#[cfg(any(feature = "unstable-plugins", docsrs))]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable-plugins")))]
pub use pipeline::{DotnetLibrary, DotnetLibraryBuilder};
pub use pipeline::{RustLibrary, RustLibraryBuilder};
