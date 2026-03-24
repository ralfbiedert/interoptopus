//! .NET runtime loader for 'reverse interop' plugins.
//!
//! Re-exports from [`interoptopus_csharp_rt`].

#[cfg(feature = "unstable-plugins")]
pub use interoptopus_csharp_rt::{DotNetError, DotNetRuntime, runtime};
