#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

pub use error::Error;

mod error;
pub mod ffi;
pub mod inventory;
pub mod lang;
pub mod pattern;
#[doc(hidden)]
#[cfg(feature = "unstable-plugins")]
pub mod plugin;
#[cfg(feature = "unstable-plugins")]
#[doc(hidden)]
pub use plugin::trampoline;
pub mod wire;

#[doc(hidden)]
pub mod proc;

#[cfg(feature = "macros")]
pub use proc::{ffi, AsyncRuntime};

#[cfg(all(feature = "macros", feature = "unstable-plugins"))]
pub use proc::plugin;

#[cfg(feature = "tokio")]
pub mod rt;
