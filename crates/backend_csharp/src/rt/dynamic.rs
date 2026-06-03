//! Singleton .NET runtime and assembly loader for Interoptopus.
//!
//! Provides a lazily-initialized, process-global [`DotnetRuntime`] via [`runtime()`].
//! Plugins are loaded as singletons via [`DotnetRuntime::load`] — each plugin
//! type and DLL path may only be used once.
//!
//! The .NET CLR can only be loaded once per process, so this crate enforces that
//! constraint by exposing a single shared instance.

use super::error::RuntimeError;
use super::shared::PluginCache;

use interoptopus::lang::plugin::{Plugin as PluginTrait, PluginLoadError};
use interoptopus::plugin::exception;
use interoptopus::trampoline::{TRAMPOLINE_UNCAUGHT_EXCEPTION, TRAMPOLINE_UNCAUGHT_EXCEPTION_CTX};
use netcorehost::hostfxr::{HostfxrContext, InitializedForRuntimeConfig};
use netcorehost::nethost;
use netcorehost::pdcstring::PdCString;
use std::path::Path;
use std::sync::{Arc, Mutex, OnceLock};

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

struct Inner {
    context: HostfxrContext<InitializedForRuntimeConfig>,
    plugins: PluginCache,
}

/// .NET runtime that can load plugin DLLs.
///
/// Only one instance can exist per process (CLR limitation). Use [`runtime()`]
/// to obtain the shared singleton. Plugins are loaded as singletons via
/// [`load`](Self::load).
pub struct DotnetRuntime {
    inner: Mutex<Inner>,
    _temp_dir: tempfile::TempDir,
}

// SAFETY: All mutable state is behind a Mutex. The raw pointers inside
// HostfxrContext prevent auto-impl but all access is serialized.
unsafe impl Send for DotnetRuntime {}
unsafe impl Sync for DotnetRuntime {}

impl DotnetRuntime {
    fn new() -> Result<Self, RuntimeError> {
        let temp_dir = tempfile::tempdir()?;
        let config_path = temp_dir.path().join("interoptopus.runtimeconfig.json");

        std::fs::write(&config_path, DEFAULT_RUNTIME_CONFIG)?;

        let fxr = nethost::load_hostfxr()?;
        let config_pdc = PdCString::from_os_str(config_path.as_os_str()).expect("temp path contains null bytes");
        let context = fxr.initialize_for_runtime_config(config_pdc)?;

        let inner = Mutex::new(Inner { context, plugins: PluginCache::new() });

        Ok(Self { inner, _temp_dir: temp_dir })
    }

    /// Loads a plugin of type `T` from the given DLL path.
    ///
    /// The same type `T` may be loaded from multiple paths, yielding independent instances.
    /// Calling with the same `(T, path)` pair returns the previously loaded instance.
    ///
    /// # Errors
    /// Can fail if `path` was previously loaded for a different plugin type.
    pub fn load<T: PluginTrait + Send + Sync + 'static>(&self, dll_path: impl AsRef<Path>) -> Result<super::Plugin<T>, PluginLoadError> {
        let path = dll_path.as_ref().to_path_buf();

        let delegate_loader = {
            let inner = self.inner.lock().expect("runtime mutex poisoned");
            inner.plugins.check_uniqueness::<T>(&path)?;
            if let Some(arc) = inner.plugins.get_cached::<T>(&path) {
                return Ok(super::Plugin::new(arc));
            }
            let dll_pdc = PdCString::from_os_str(path.as_os_str()).expect("dll path contains null bytes");
            inner
                .context
                .get_delegate_loader_for_assembly(dll_pdc)
                .map_err(|e| PluginLoadError::load_failed(e.to_string()))?
        };

        let assembly_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| PluginLoadError::load_failed("invalid DLL path".to_string()))?
            .to_string();

        let type_name = format!("Interoptopus.API.Interop, {assembly_name}");

        let plugin = T::load_from(|symbol| {
            let type_pdc = PdCString::from_os_str(type_name.as_ref() as &std::ffi::OsStr).expect("type name contains null bytes");
            let method_pdc = PdCString::from_os_str(symbol.as_ref() as &std::ffi::OsStr).expect("symbol name contains null bytes");
            match delegate_loader.get_function_with_unmanaged_callers_only::<extern "system" fn()>(&type_pdc, &method_pdc) {
                Ok(managed_fn) => {
                    let f: extern "system" fn() = *managed_fn;
                    f as *const u8
                }
                Err(_) => std::ptr::null(),
            }
        })?;

        // Register wire buffer trampolines.
        let register_fn = plugin.register_trampoline_fn();
        interoptopus::register_wire_trampolines!(|id, ptr| {
            (register_fn)(id, ptr);
        });

        // Register the uncaught-exception sink. The plugin's outer try/catch will call
        // this on the calling thread; generated `plugin!` wrappers panic if the
        // thread-local slot was set during the most recent invocation.
        register_fn(TRAMPOLINE_UNCAUGHT_EXCEPTION, exception::callback_ptr());
        register_fn(TRAMPOLINE_UNCAUGHT_EXCEPTION_CTX, std::ptr::null());

        // Verify API guard after trampolines are registered so the query function works.
        plugin.verify_api_guard()?;

        let arc = Arc::new(plugin);
        {
            let mut inner = self.inner.lock().expect("runtime mutex poisoned");
            inner.plugins.insert::<T>(path, Arc::clone(&arc));
        }
        Ok(super::Plugin::new(arc))
    }
}

static RUNTIME: OnceLock<Result<DotnetRuntime, String>> = OnceLock::new();

/// Returns the process-global .NET runtime.
///
/// The .NET CLR can only be loaded once per process. This function lazily
/// creates the singleton and returns a shared reference on every subsequent call.
///
/// # Errors
///
/// Returns [`RuntimeError`] if the runtime failed to initialize.
/// Once successfully initialized, all subsequent calls return the same instance.
pub fn runtime() -> Result<&'static DotnetRuntime, RuntimeError> {
    RUNTIME
        .get_or_init(|| DotnetRuntime::new().map_err(|e| e.to_string()))
        .as_ref()
        .map_err(|msg| RuntimeError::from(msg.clone()))
}
