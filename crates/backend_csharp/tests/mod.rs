#![allow(unused)]

use interoptopus_csharp::pattern::Exception;
use std::path::{Path, PathBuf};

#[macro_use]
mod common;
mod output;
mod plugins;
mod reference_plugins;

pub const FILE_NOT_FOUND_EXCEPTION: Exception = Exception::new("System.IO.FileNotFoundException");

mod reference_project {
    use interoptopus_csharp::config::HeaderConfig;
    use interoptopus_csharp::dispatch::Dispatch;
    use interoptopus_csharp::lang::meta::FileEmission;
    use interoptopus_csharp::output::Target;
    use interoptopus_csharp::RustLibrary;

    #[test]
    fn interop() -> Result<(), Box<dyn std::error::Error>> {
        let multibuf = RustLibrary::builder(reference_project::inventory())
            .dll_name("reference_project")
            .dispatch(Dispatch::custom(|x, _| match x.emission {
                FileEmission::Common => Target::new("Interop.Common.cs", "My.Company.Common"),
                FileEmission::Default => Target::new("Interop.cs", "My.Company"),
                FileEmission::CustomModule(_) => Target::new("Interop.cs", "My.Company"),
            }))
            .header_config(HeaderConfig { emit_version: false })
            .build()
            .process()?;

        multibuf.write_buffers_to("tests/reference_project")?;
        multibuf.write_buffers_to("benches/dotnet")?;

        // insta::assert_snapshot!(multibuf);

        Ok(())
    }
}

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
