#![cfg(feature = "unstable-plugins")]

use interoptopus_backends::output::Multibuf;
use std::path::{Path, PathBuf};

mod complex;
mod functions;
mod pattern;
mod service;
mod wire;

/// Generates interop files for `$plugin` into the `$name` folder and snapshot-tests the output.
#[macro_export]
macro_rules! define_plugin {
    ($plugin:ty, $name:expr) => {{
        let multibuf = ::interoptopus_csharp::DotnetLibrary::builder(<$plugin as ::interoptopus::lang::plugin::PluginInfo>::inventory())
            .build()
            .process()?;

        multibuf.write_buffers_to(super::interop_path_for($name))?;
        insta::assert_snapshot!(multibuf);
    }};
}

/// Loads a pre-built `$name.dll` plugin and returns an instance of `$plugin` with resolved function pointers.
#[macro_export]
macro_rules! load_plugin {
    ($plugin:ty, $name:expr) => {{
        let loader = ::interoptopus_csharp::plugin::DotNetRuntime::new()?
            .set_exception_handler(|x| println!("{x}"))
            .dll_loader_with_namespace(super::plugin_path_for($name), "My.Company")?;
        <$plugin>::new(&loader)?
    }};
}

/// Returns the path to a compiled plugin DLL.
fn plugin_path_for(path: impl AsRef<Path>) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/reference_plugins/_plugins").join(path.as_ref())
}

/// Returns the path to the interop output folder for a given plugin.
fn interop_path_for(path: impl AsRef<Path>) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/reference_plugins/").join(path.as_ref())
}
