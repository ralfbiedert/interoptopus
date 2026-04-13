// #![allow(unused)]

use interoptopus_csharp::pattern::Exception;
use std::path::{Path, PathBuf};

#[macro_use]
mod common;
mod backend_plugins;
mod output;
mod reference_plugins;
mod reference_project;

pub const FILE_NOT_FOUND_EXCEPTION: Exception = Exception::new("System.IO.FileNotFoundException");

mod model {
    mod service_rval_result;
}

/// Generates interop files for `$plugin` into the `$base/$name` folder and snapshot-tests the output.
#[macro_export]
macro_rules! define_plugin {
    ($plugin:ty, $name:expr, $base:expr) => {{
        use interoptopus_csharp::dispatch::Dispatch;

        let multibuf = ::interoptopus_csharp::DotnetLibrary::builder(<$plugin as ::interoptopus::lang::plugin::PluginInfo>::inventory())
            .dispatch(Dispatch::plugin_defaults_with("My.Company"))
            .exception(crate::FILE_NOT_FOUND_EXCEPTION)
            .build()
            .process()?;

        let base = ::std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join($base);
        multibuf.write_buffers_to(base.join($name))?;
        insta::assert_snapshot!(multibuf);
    }};
}

/// Loads a pre-built `$name.dll` plugin from `$base/_plugins/` and returns an instance of `$plugin`.
#[macro_export]
macro_rules! load_plugin {
    ($plugin:ty, $name:expr, $base:expr) => {{
        let rt = ::interoptopus_csharp::rt::dynamic::runtime().expect("failed to initialize .NET runtime");
        let path = ::std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join($base).join("_plugins").join($name);
        rt.load::<$plugin>(path)?
    }};
}

/// Returns the path to a compiled plugin DLL under the given base directory.
fn dll_path_for(base: impl AsRef<Path>, name: impl AsRef<Path>) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(base).join("_plugins").join(name)
}
