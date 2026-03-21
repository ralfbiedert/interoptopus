#![cfg(feature = "unstable-plugins")]

use interoptopus_backends::output::Multibuf;
use std::path::{Path, PathBuf};

mod basic;
mod old;

#[macro_export]
macro_rules! define_check_and_load_plugin {
    ($plugin:ty, $name:expr) => {{
        let multibuf = ::interoptopus_csharp::DotnetLibrary::builder(<$plugin as ::interoptopus::lang::plugin::PluginInfo>::inventory())
            .build()
            .process()?;

        multibuf.write_buffers_to(super::interop_path_for($name))?;

        insta::assert_snapshot!(multibuf);

        let loader = ::interoptopus_csharp::plugin::DotNetRuntime::new()?.dll_loader_with_namespace(super::plugin_path_for($name), "My.Company")?;
        <$plugin>::new(&loader)?
    }};
}

fn plugin_path_for(path: impl AsRef<Path>) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/reference_plugins/_plugins").join(path.as_ref())
}

fn interop_path_for(path: impl AsRef<Path>) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/reference_plugins/").join(path.as_ref())
}
