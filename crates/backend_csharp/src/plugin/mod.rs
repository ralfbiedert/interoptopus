//! .NET runtime loader for 'reverse interop' plugins.

#[cfg(feature = "unstable-plugins")]
mod runtime;

#[cfg(feature = "unstable-plugins")]
pub use runtime::{DllLoader, DotNetError, DotNetRuntime};
