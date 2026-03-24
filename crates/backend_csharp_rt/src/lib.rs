//! Singleton .NET runtime and assembly loader for Interoptopus.
//!
//! Provides a lazily-initialized, process-global [`DotNetRuntime`] via [`runtime()`].
//! Plugins are loaded as singletons via [`DotNetRuntime::load`] — each plugin
//! type and DLL path may only be used once.
//!
//! The .NET CLR can only be loaded once per process, so this crate enforces that
//! constraint by exposing a single shared instance.

use interoptopus::lang::plugin::{Plugin, PluginLoadError};
use interoptopus::trampoline::{TRAMPOLINE_UNCAUGHT_EXCEPTION, TRAMPOLINE_UNCAUGHT_EXCEPTION_CTX};
use netcorehost::hostfxr::{HostfxrContext, InitializedForRuntimeConfig};
use netcorehost::nethost;
use netcorehost::pdcstring::PdCString;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};
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
    plugins: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    type_to_path: HashMap<TypeId, PathBuf>,
    path_to_type: HashMap<PathBuf, TypeId>,
}

/// A loaded .NET runtime.
///
/// Only one instance can exist per process (CLR limitation). Use [`runtime()`]
/// to obtain the shared singleton. Plugins are loaded as singletons via
/// [`load_from`](Self::load).
pub struct DotNetRuntime {
    inner: Mutex<Inner>,
    exception_handler: OnceLock<Arc<dyn Fn(String) + Send + Sync>>,
    _temp_dir: tempfile::TempDir,
}

// SAFETY: All mutable state is behind a Mutex. The raw pointers inside
// HostfxrContext prevent auto-impl but all access is serialized.
unsafe impl Send for DotNetRuntime {}
unsafe impl Sync for DotNetRuntime {}

/// Errors that can occur when initializing the .NET runtime.
#[derive(Debug)]
pub enum DotNetError {
    HostfxrLoad(netcorehost::nethost::LoadHostfxrError),
    RuntimeInit(netcorehost::error::HostingError),
    Io(std::io::Error),
    InitFailed(String),
}

impl fmt::Display for DotNetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HostfxrLoad(e) => write!(f, "failed to load hostfxr: {e}"),
            Self::RuntimeInit(e) => write!(f, "failed to initialize .NET runtime: {e}"),
            Self::Io(e) => write!(f, "IO error: {e}"),
            Self::InitFailed(msg) => write!(f, "runtime initialization failed previously: {msg}"),
        }
    }
}

impl std::error::Error for DotNetError {}

impl DotNetRuntime {
    fn new() -> Result<Self, DotNetError> {
        let temp_dir = tempfile::tempdir().map_err(DotNetError::Io)?;
        let config_path = temp_dir.path().join("interoptopus.runtimeconfig.json");

        std::fs::write(&config_path, DEFAULT_RUNTIME_CONFIG).map_err(DotNetError::Io)?;

        let fxr = nethost::load_hostfxr().map_err(DotNetError::HostfxrLoad)?;
        let config_pdc = PdCString::from_os_str(config_path.as_os_str()).expect("temp path contains null bytes");
        let context = fxr.initialize_for_runtime_config(config_pdc).map_err(DotNetError::RuntimeInit)?;

        let inner = Mutex::new(Inner { context, plugins: HashMap::new(), type_to_path: HashMap::new(), path_to_type: HashMap::new() });

        Ok(Self { inner, exception_handler: OnceLock::new(), _temp_dir: temp_dir })
    }

    /// Sets the global exception handler called when a plugin reports an uncaught exception.
    ///
    /// May only be called once. Panics if called a second time.
    pub fn exception_handler(&self, handler: impl Fn(String) + Send + Sync + 'static) {
        if self.exception_handler.set(Arc::new(handler)).is_err() {
            panic!("exception handler already set");
        }
    }

    /// Loads a plugin of type `T` from the given DLL path, returning a cached singleton.
    ///
    /// Each plugin type `T` and each DLL path may only be used in one combination.
    /// Calling with the same `(T, path)` returns the previously loaded instance.
    ///
    /// # Panics
    ///
    /// - If `T` was previously loaded from a different path.
    /// - If `path` was previously loaded for a different plugin type.
    pub fn load<T: Plugin + Send + Sync + 'static>(&self, dll_path: impl AsRef<Path>) -> Result<&T, PluginLoadError> {
        let type_id = TypeId::of::<T>();
        let path = dll_path.as_ref().to_path_buf();

        let mut inner = self.inner.lock().expect("runtime mutex poisoned");

        // Enforce uniqueness: each T maps to exactly one path and vice versa.
        if let Some(existing_path) = inner.type_to_path.get(&type_id) {
            assert!(*existing_path == path, "plugin {} already loaded from {:?}, cannot load from {:?}", std::any::type_name::<T>(), existing_path, path,);
        }
        if let Some(existing_type) = inner.path_to_type.get(&path) {
            assert!(*existing_type == type_id, "DLL {:?} already loaded for a different plugin type", path,);
        }

        // Return cached instance.
        if let Some(boxed) = inner.plugins.get(&type_id) {
            let reference = boxed.downcast_ref::<T>().expect("type mismatch in plugin cache");
            // SAFETY: The Box is heap-allocated inside a HashMap in the 'static singleton.
            // Entries are never removed and the HashMap's growth only moves the Box pointer
            // (a thin wrapper around a heap pointer), not the heap data it points to.
            return Ok(unsafe { &*(reference as *const T) });
        }

        // Load assembly and resolve symbols.
        let dll_pdc = PdCString::from_os_str(path.as_os_str()).expect("dll path contains null bytes");
        let delegate_loader = inner
            .context
            .get_delegate_loader_for_assembly(dll_pdc)
            .map_err(|e| PluginLoadError::LoadFailed(e.to_string()))?;

        let assembly_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| PluginLoadError::LoadFailed("invalid DLL path".to_string()))?
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

        // Register exception handler if set.
        if let Some(handler) = self.exception_handler.get() {
            let ctx = Box::into_raw(Box::new(HandlerShim { handler: Arc::clone(handler) })) as *const u8;
            register_fn(TRAMPOLINE_UNCAUGHT_EXCEPTION, uncaught_exception_callback as *const u8);
            register_fn(TRAMPOLINE_UNCAUGHT_EXCEPTION_CTX, ctx);
        }

        // Cache and return.
        inner.type_to_path.insert(type_id, path.clone());
        inner.path_to_type.insert(path, type_id);
        inner.plugins.insert(type_id, Box::new(plugin));

        let reference = inner.plugins.get(&type_id).unwrap().downcast_ref::<T>().unwrap();
        // SAFETY: Same as above — heap-allocated, never removed, lives in a 'static singleton.
        Ok(unsafe { &*(reference as *const T) })
    }
}

static RUNTIME: OnceLock<Result<DotNetRuntime, String>> = OnceLock::new();

/// Returns the process-global .NET runtime, initializing it on first call.
///
/// The .NET CLR can only be loaded once per process. This function lazily
/// creates the singleton and returns a shared reference on every subsequent call.
///
/// # Errors
///
/// Returns [`DotNetError`] if the runtime failed to initialize.
/// Once successfully initialized, all subsequent calls return the same instance.
pub fn runtime() -> Result<&'static DotNetRuntime, DotNetError> {
    RUNTIME
        .get_or_init(|| DotNetRuntime::new().map_err(|e| e.to_string()))
        .as_ref()
        .map_err(|msg| DotNetError::InitFailed(msg.clone()))
}

/// Concrete Sized wrapper so we can store the trait-object handler behind a thin pointer.
struct HandlerShim {
    handler: Arc<dyn Fn(String) + Send + Sync>,
}

/// Callback registered with the managed plugin for uncaught exceptions.
#[allow(clippy::cast_ptr_alignment)]
unsafe extern "C" fn uncaught_exception_callback(ctx: *const u8, message: *const u8, len: i32) {
    let shim = unsafe { &*ctx.cast::<HandlerShim>() };
    let bytes = unsafe { std::slice::from_raw_parts(message, len.unsigned_abs() as usize) };
    let msg = String::from_utf8_lossy(bytes).into_owned();
    (shim.handler)(msg);
}
