//! .NET runtime loader for 'reverse interop' plugins.
mod runtime;

pub use runtime::{DllLoader, DotNetError, DotNetRuntime};
