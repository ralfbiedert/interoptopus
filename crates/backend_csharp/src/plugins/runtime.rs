use netcorehost::hostfxr::{HostfxrContext, InitializedForRuntimeConfig};
use netcorehost::nethost;
use std::fmt;
use std::sync::Arc;

const DEFAULT_RUNTIME_CONFIG: &str = r#"{
  "runtimeOptions": {
    "tfm": "net9.0",
    "framework": {
      "name": "Microsoft.NETCore.App",
      "version": "9.0.0"
    }
  }
}"#;

/// A loaded .NET runtime that can be used to load plugin assemblies.
pub struct DotNetRuntime {
    context: Arc<HostfxrContext<InitializedForRuntimeConfig>>,
    _temp_dir: tempfile::TempDir,
}

/// Errors that can occur when working with the .NET runtime.
#[derive(Debug)]
pub enum DotNetError {
    HostfxrLoad(netcorehost::nethost::LoadHostfxrError),
    RuntimeInit(netcorehost::error::HostingError),
    Io(std::io::Error),
}

impl fmt::Display for DotNetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HostfxrLoad(e) => write!(f, "failed to load hostfxr: {e}"),
            Self::RuntimeInit(e) => write!(f, "failed to initialize .NET runtime: {e}"),
            Self::Io(e) => write!(f, "IO error: {e}"),
        }
    }
}

impl std::error::Error for DotNetError {}

impl DotNetRuntime {
    /// Creates a new .NET runtime using the default embedded runtime config.
    pub fn new() -> Result<Self, DotNetError> {
        let temp_dir = tempfile::tempdir().map_err(DotNetError::Io)?;
        let config_path = temp_dir.path().join("interoptopus.runtimeconfig.json");

        std::fs::write(&config_path, DEFAULT_RUNTIME_CONFIG).map_err(DotNetError::Io)?;

        let fxr = nethost::load_hostfxr().map_err(DotNetError::HostfxrLoad)?;

        let config_pdc = netcorehost::pdcstring::PdCString::from_os_str(config_path.as_os_str())
            .expect("temp path contains null bytes");

        let context = fxr
            .initialize_for_runtime_config(config_pdc)
            .map_err(DotNetError::RuntimeInit)?;

        Ok(Self {
            context: Arc::new(context),
            _temp_dir: temp_dir,
        })
    }

    /// Returns a reference to the underlying runtime context.
    pub fn context(&self) -> &HostfxrContext<InitializedForRuntimeConfig> {
        &self.context
    }
}
