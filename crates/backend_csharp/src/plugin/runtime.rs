use interoptopus::lang::plugin::{Loader, Plugin, PluginLoadError};
use interoptopus::trampoline::TRAMPOLINE_UNCAUGHT_EXCEPTION;
use netcorehost::hostfxr::{AssemblyDelegateLoader, HostfxrContext, InitializedForRuntimeConfig};
use netcorehost::nethost;
use netcorehost::pdcstring::PdCString;
use std::fmt;
use std::path::Path;
use std::sync::{Arc, RwLock};

const DEFAULT_RUNTIME_CONFIG: &str = r#"{
  "runtimeOptions": {
    "tfm": "net10.0",
    "rollForward": "LatestMajor",                                                                                                                                                                                                                                        
    "framework": {
      "name": "Microsoft.NETCore.App",
      "version": "10.0.0"
    }
  }
}"#;

/// Process-global handler for uncaught plugin exceptions.
///
/// Updated by [`DotNetRuntime::set_exception_handler`] before a plugin is loaded.
/// Called from the `extern "C"` trampoline below, which is registered with
/// the managed side via `register_trampoline`.
static UNCAUGHT_EXCEPTION_HANDLER: RwLock<Option<Arc<dyn Fn(String) + Send + Sync>>> = RwLock::new(None);

extern "C" fn uncaught_exception_callback(message: *const u8, len: i32) {
    let guard = UNCAUGHT_EXCEPTION_HANDLER.read().unwrap_or_else(|e| e.into_inner());
    if let Some(handler) = guard.as_ref() {
        let bytes = unsafe { std::slice::from_raw_parts(message, len as usize) };
        let msg = String::from_utf8_lossy(bytes).into_owned();
        handler(msg);
    }
}

/// A loaded .NET runtime that can be used to load plugin assemblies.
pub struct DotNetRuntime {
    context: Arc<HostfxrContext<InitializedForRuntimeConfig>>,
    _temp_dir: tempfile::TempDir,
    exception_handler: Option<Arc<dyn Fn(String) + Send + Sync>>,
}

/// A symbol loader bound to a specific .NET assembly.
///
/// Created via [`DotNetRuntime::dll_loader`]. Implements [`Loader`] so it can
/// be passed to plugin constructors (e.g., `MathPlugin::new(&loader)`).
pub struct DllLoader {
    delegate_loader: AssemblyDelegateLoader,
    type_name: String,
    exception_handler: Option<Arc<dyn Fn(String) + Send + Sync>>,
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

        Ok(Self { context: Arc::new(context), _temp_dir: temp_dir, exception_handler: None })
    }

    /// Sets a handler that is called whenever the plugin reports an uncaught exception.
    ///
    /// The handler receives the exception message as a `String`. It is invoked on whatever
    /// thread the trampoline ran on, so it must be `Send + Sync`.
    #[must_use]
    pub fn set_exception_handler(mut self, handler: impl Fn(String) + Send + Sync + 'static) -> Self {
        self.exception_handler = Some(Arc::new(handler));
        self
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
    pub fn dll_loader_with_namespace(&self, dll_path: impl AsRef<Path>, namespace: &str) -> Result<DllLoader, PluginLoadError> {
        let dll_pdc = PdCString::from_os_str(dll_path.as_ref().as_os_str()).expect("dll path contains null bytes");

        let delegate_loader = self
            .context
            .get_delegate_loader_for_assembly(dll_pdc)
            .map_err(|e| PluginLoadError::LoadFailed(e.to_string()))?;

        let assembly_name = dll_path
            .as_ref()
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| PluginLoadError::LoadFailed("invalid DLL path".to_string()))?
            .to_string();

        let type_name = format!("{namespace}.Interop, {assembly_name}");

        Ok(DllLoader { delegate_loader, type_name, exception_handler: self.exception_handler.clone() })
    }

    /// Returns a reference to the underlying runtime context.
    pub fn context(&self) -> &HostfxrContext<InitializedForRuntimeConfig> {
        &self.context
    }
}

impl DllLoader {
    fn load_symbol(&self, symbol: &str) -> *const u8 {
        let type_pdc = PdCString::from_os_str(self.type_name.as_ref() as &std::ffi::OsStr).expect("type name contains null bytes");
        let method_pdc = PdCString::from_os_str(symbol.as_ref() as &std::ffi::OsStr).expect("symbol name contains null bytes");
        match self.delegate_loader.get_function_with_unmanaged_callers_only::<extern "system" fn()>(&type_pdc, &method_pdc) {
            Ok(managed_fn) => {
                let f: extern "system" fn() = *managed_fn;
                unsafe { std::mem::transmute::<_, *const u8>(f) }
            }
            Err(_) => std::ptr::null(),
        }
    }
}

impl Loader for DllLoader {
    fn load_plugin<T: Plugin>(&self) -> Result<T, PluginLoadError> {
        // If an exception handler was provided, install it in the global slot before loading.
        if let Some(handler) = &self.exception_handler {
            let mut guard = UNCAUGHT_EXCEPTION_HANDLER.write().unwrap_or_else(|e| e.into_inner());
            *guard = Some(Arc::clone(handler));
        }

        let plugin = T::load_from(|symbol_name| self.load_symbol(symbol_name))?;

        // Register the uncaught-exception callback with the managed side.
        if self.exception_handler.is_some() {
            let register_ptr = self.load_symbol("register_trampoline");
            if !register_ptr.is_null() {
                let register_fn: extern "C" fn(i64, *const u8) = unsafe { std::mem::transmute(register_ptr) };
                register_fn(TRAMPOLINE_UNCAUGHT_EXCEPTION, uncaught_exception_callback as *const u8);
            }
        }

        Ok(plugin)
    }
}
