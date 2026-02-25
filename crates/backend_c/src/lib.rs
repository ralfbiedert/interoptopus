#![doc = include_str!("../README.md")]

pub mod lang;
pub mod pass;
pub mod pipeline;

pub use pipeline::{CLibrary, CLibraryBuilder};

use interoptopus::inventory::RustInventory;
use std::fs;
use std::io;
use std::path::Path;

/// Convenience shorthand: build + process + write a single `.h` file.
pub fn generate(loader_name: &str, inv: &RustInventory, path: impl AsRef<Path>) -> Result<(), io::Error> {
    let filename = path
        .as_ref()
        .file_name()
        .map_or_else(|| format!("{loader_name}.h"), |s| s.to_string_lossy().to_string());

    let multibuf = CLibrary::builder(inv)
        .loader_name(loader_name)
        .filename(&filename)
        .build()
        .process()
        .map_err(io::Error::other)?;

    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }

    multibuf.write_buffer_to(path.as_ref().parent().unwrap_or_else(|| Path::new(".")), &filename)?;
    Ok(())
}
