#![cfg_attr(docsrs, feature(doc_cfg))]
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

pub use error::Error;
#[cfg(any(feature = "unstable-plugins", docsrs))]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable-plugins")))]
#[doc(inline)]
pub use interoptopus_csharp_rt::aot::{runtime as aot_runtime, AotRuntime};
#[cfg(any(feature = "unstable-plugins", docsrs))]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable-plugins")))]
#[doc(inline)]
pub use interoptopus_csharp_rt::dynamic::{runtime as dotnet_runtime, DotnetRuntime};
#[cfg(any(feature = "unstable-plugins", docsrs))]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable-plugins")))]
pub use pipeline::{DotnetLibrary, DotnetLibraryBuilder};
pub use pipeline::{RustLibrary, RustLibraryBuilder};
