//! Test helpers for C bindings.

use anyhow::Error;
use std::path::Path;

/// Compile the given C app, ignore and succeed otherwise.
#[cfg(target_os = "windows")]
pub fn compile_c_app_if_installed(file: impl AsRef<Path>) -> Result<(), Error> {
    let file = Path::new("tests").join(file);
    let temp_dir = tempdir()?;

    cc::Build::new()
        .file(file)
        .out_dir(temp_dir.path())
        .warnings(false)
        .target("x86_64-pc-windows-msvc")
        .opt_level(0)
        .host("x86_64-pc-windows-msvc")
        .try_compile("foo")?;

    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn compile_c_app_if_installed(_app: impl AsRef<Path>) -> Result<(), Error> {
    Ok(())
}

#[macro_export]
macro_rules! compile_output_c {
    ($generated:expr) => {{
        if !$crate::UPDATE_BINDINGS {
            let temp_dir = $crate::tempdir()?;
            let header_file = temp_dir.path().join("header.h");
            let c_file = temp_dir.path().join("app.c");

            std::fs::write(header_file, $generated)?;
            std::fs::write(
                c_file.clone(),
                r#"
                #include "header.h"
                void main() {}
            "#,
            )?;
            $crate::backend_c::compile_c_app_if_installed(c_file)?;
        }
    }};
}
