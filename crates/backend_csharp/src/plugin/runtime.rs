use interoptopus::lang::plugin::{Loader, Plugin, PluginLoadError};
use interoptopus::trampoline::{TRAMPOLINE_UNCAUGHT_EXCEPTION, TRAMPOLINE_UNCAUGHT_EXCEPTION_CTX};
use netcorehost::hostfxr::{AssemblyDelegateLoader, HostfxrContext, InitializedForRuntimeConfig};
use netcorehost::nethost;
use netcorehost::pdcstring::PdCString;
use std::fmt;
use std::path::Path;
use std::sync::Arc;

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

/// Concrete Sized wrapper so we can store the trait-object handler behind a thin pointer.
struct HandlerShim {
    handler: Arc<dyn Fn(String) + Send + Sync>,
}

/// Callback registered with the managed plugin for uncaught exceptions.
///
/// `ctx` is a `*mut HandlerShim` produced by [`Box::into_raw`] in
/// [`DllLoader::load_plugin`] and intentionally leaked.
#[allow(clippy::cast_ptr_alignment)]
unsafe extern "C" fn uncaught_exception_callback(ctx: *const u8, message: *const u8, len: i32) {
    // SAFETY: ctx was produced by Box::into_raw(Box::new(HandlerShim { ... })) in
    // load_plugin and intentionally leaked, so the pointer is valid and properly
    // aligned for the lifetime of the process. Only this callback ever reads it.
    let shim = unsafe { &*ctx.cast::<HandlerShim>() };
    // SAFETY: message is a UTF-8 byte slice of length len provided by the managed
    // side inside a `fixed` block, so it is valid and pinned for the duration of
    // this call. len is non-negative (unsigned_abs guarantees that).
    let bytes = unsafe { std::slice::from_raw_parts(message, len.unsigned_abs() as usize) };
    let msg = String::from_utf8_lossy(bytes).into_owned();
    (shim.handler)(msg);
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

        #[allow(clippy::arc_with_non_send_sync)]
        let context = Arc::new(context);
        Ok(Self { context, _temp_dir: temp_dir, exception_handler: None })
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
    /// Symbols are resolved from `Interoptopus.API.Interop` in the assembly.
    pub fn dll_loader(&self, dll_path: impl AsRef<Path>) -> Result<DllLoader, PluginLoadError> {
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

        let type_name = format!("Interoptopus.API.Interop, {assembly_name}");

        Ok(DllLoader { delegate_loader, type_name, exception_handler: self.exception_handler.clone() })
    }

    /// Returns a reference to the underlying runtime context.
    #[must_use]
    pub fn context(&self) -> &HostfxrContext<InitializedForRuntimeConfig> {
        &self.context
    }
}

impl DllLoader {
    fn load_symbol(&self, symbol: &str) -> *const u8 {
        let type_pdc = PdCString::from_os_str(self.type_name.as_ref() as &std::ffi::OsStr).expect("type name contains null bytes");
        let method_pdc = PdCString::from_os_str(symbol.as_ref() as &std::ffi::OsStr).expect("symbol name contains null bytes");
        match self
            .delegate_loader
            .get_function_with_unmanaged_callers_only::<extern "system" fn()>(&type_pdc, &method_pdc)
        {
            Ok(managed_fn) => {
                let f: extern "system" fn() = *managed_fn;
                f as *const u8
            }
            Err(_) => std::ptr::null(),
        }
    }
}

impl Loader for DllLoader {
    fn load_plugin<T: Plugin>(&self) -> Result<T, PluginLoadError> {
        let plugin = T::load_from(|symbol_name| self.load_symbol(symbol_name))?;

        // Register the uncaught-exception callback and its context pointer with the
        // managed side. The context is a heap-allocated Arc clone that the callback
        // reads on every invocation — no global state involved.
        if let Some(handler) = &self.exception_handler {
            let register_fn = plugin.register_trampoline_fn();
            let ctx = Box::into_raw(Box::new(HandlerShim { handler: Arc::clone(handler) })) as *const u8;
            register_fn(TRAMPOLINE_UNCAUGHT_EXCEPTION, uncaught_exception_callback as *const u8);
            register_fn(TRAMPOLINE_UNCAUGHT_EXCEPTION_CTX, ctx);
        }

        Ok(plugin)
    }
}
