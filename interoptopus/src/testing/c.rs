use std::path::Path;
use crate::Error;

/// If `dotnet` is installed, run the command as `dotnet command` from `path`, ignore and succeed otherwise.
pub fn compile_c_app_if_installed<P: AsRef<Path>>(out_dir: P, app: &str) -> Result<(), Error> {
    // TODO, better handling of target, ...
    cc::Build::new().file(app).out_dir(out_dir).target("x86_64-pc-windows-msvc").opt_level(0).host("x86_64-pc-windows-msvc").try_compile("foo").unwrap();

    Ok(())
}