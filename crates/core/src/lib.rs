#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

pub use error::Error;

mod error;
pub mod ffi;
pub mod inventory;
pub mod lang;
pub mod pattern;
pub mod wire;

#[doc(hidden)]
pub mod proc;

#[cfg(feature = "derive")]
pub use proc::{ffi, AsyncRuntime};

#[cfg(feature = "tokio")]
pub mod rt;
