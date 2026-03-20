use interoptopus::lang::plugin::{Loader, Plugin, PluginLoadError};
use netcorehost::hostfxr::{AssemblyDelegateLoader, HostfxrContext, InitializedForRuntimeConfig};
use netcorehost::nethost;
use netcorehost::pdcstring::PdCString;
use std::fmt;
use std::path::Path;
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

/// A symbol loader bound to a specific .NET assembly.
///
/// Created via [`DotNetRuntime::dll_loader`]. Implements [`Loader`] so it can
/// be passed to plugin constructors (e.g., `MathPlugin::new(&loader)`).
pub struct DllLoader {
    delegate_loader: AssemblyDelegateLoader,
    type_name: String,
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

        let config_pdc = PdCString::from_os_str(config_path.as_os_str()).expect("temp path contains null bytes");

        let context = fxr.initialize_for_runtime_config(config_pdc).map_err(DotNetError::RuntimeInit)?;

        Ok(Self {
            context: Arc::new(context),
            _temp_dir: temp_dir,
        })
    }

    /// Creates a [`DllLoader`] bound to the given assembly.
    ///
    /// By default, symbols are resolved from `{Assembly}.Interop` in the assembly.
    /// Use [`dll_loader_with_namespace`](Self::dll_loader_with_namespace) to
    /// specify a custom namespace.
    pub fn dll_loader(&self, path: &str) -> Result<DllLoader, PluginLoadError> {
        let assembly_name = Path::new(path)
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| PluginLoadError::LoadFailed("invalid DLL path".to_string()))?
            .to_string();

        self.dll_loader_with_namespace(path, &assembly_name)
    }

    /// Creates a [`DllLoader`] bound to the given assembly, looking up
    /// `[UnmanagedCallersOnly]` methods in `{namespace}.Interop`.
    pub fn dll_loader_with_namespace(&self, path: &str, namespace: &str) -> Result<DllLoader, PluginLoadError> {
        let dll_path = Path::new(path);

        let dll_pdc = PdCString::from_os_str(dll_path.as_os_str()).expect("dll path contains null bytes");

        let delegate_loader = self
            .context
            .get_delegate_loader_for_assembly(dll_pdc)
            .map_err(|e| PluginLoadError::LoadFailed(e.to_string()))?;

        let assembly_name = dll_path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| PluginLoadError::LoadFailed("invalid DLL path".to_string()))?
            .to_string();

        let type_name = format!("{namespace}.Interop, {assembly_name}");

        Ok(DllLoader { delegate_loader, type_name })
    }

    /// Returns a reference to the underlying runtime context.
    pub fn context(&self) -> &HostfxrContext<InitializedForRuntimeConfig> {
        &self.context
    }
}

impl Loader for DllLoader {
    fn load_plugin<T: Plugin>(&self) -> Result<T, PluginLoadError> {
        T::load_from(|symbol_name| {
            let type_pdc = PdCString::from_os_str(self.type_name.as_ref() as &std::ffi::OsStr).expect("type name contains null bytes");
            let method_pdc = PdCString::from_os_str(symbol_name.as_ref() as &std::ffi::OsStr).expect("symbol name contains null bytes");

            match self.delegate_loader.get_function_with_unmanaged_callers_only::<extern "system" fn()>(&type_pdc, &method_pdc) {
                Ok(managed_fn) => {
                    let f: extern "system" fn() = *managed_fn;
                    unsafe { std::mem::transmute::<extern "system" fn(), *const u8>(f) }
                }
                Err(_) => std::ptr::null(),
            }
        })
    }
}
