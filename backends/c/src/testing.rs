//! Test helpers for C bindings.

use crate::Error;
use std::path::Path;

/// Compile the given C app, ignore and succeed otherwise.
#[cfg(all(target_os = "windows", feature = "cc"))]
pub fn compile_c_app_if_installed<P: AsRef<Path>>(out_dir: P, app: &str) -> Result<(), Error> {
    // TODO, better handling of target, ...
    cc::Build::new()
        .file(app)
        .out_dir(out_dir)
        .target("x86_64-pc-windows-msvc")
        .opt_level(0)
        .host("x86_64-pc-windows-msvc")
        .try_compile("foo")
        .unwrap();

    Ok(())
}

#[cfg(not(all(target_os = "windows", feature = "cc")))]
pub fn compile_c_app_if_installed<P: AsRef<Path>>(_out_dir: P, _app: &str) -> Result<(), Error> {
    Ok(())
}
