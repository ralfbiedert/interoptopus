//! AOT (native library) runtime loader for Interoptopus.
//!
//! Provides a lazily-initialized, process-global [`AotRuntime`] via [`runtime()`].
//! Unlike the dynamic runtime, this loads ahead-of-time compiled native libraries
//! using `libloading` instead of hosting the .NET CLR.

use super::shared::{self, HandlerShim, PluginCache};

use interoptopus::lang::plugin::{Plugin as PluginTrait, PluginLoadError};
use interoptopus::trampoline::{TRAMPOLINE_UNCAUGHT_EXCEPTION, TRAMPOLINE_UNCAUGHT_EXCEPTION_CTX};
use std::path::Path;
use std::sync::{Arc, Mutex, OnceLock};

struct Inner {
    plugins: PluginCache,
    /// Keep loaded libraries alive for the lifetime of the runtime.
    libraries: Vec<libloading::Library>,
}

/// AOT runtime that loads native plugin libraries.
///
/// Only one instance exists per process. Use [`runtime()`] to obtain the
/// shared singleton. Plugins are loaded as singletons via [`load`](Self::load).
pub struct AotRuntime {
    inner: Mutex<Inner>,
    exception_handler: OnceLock<Arc<dyn Fn(String) + Send + Sync>>,
}

// SAFETY: All mutable state is behind a Mutex.
unsafe impl Send for AotRuntime {}
unsafe impl Sync for AotRuntime {}

impl AotRuntime {
    fn new() -> Self {
        let inner = Mutex::new(Inner { plugins: PluginCache::new(), libraries: Vec::new() });
        Self { inner, exception_handler: OnceLock::new() }
    }

    /// Sets the exception handler called when a plugin reports an uncaught exception.
    ///
    /// # Panics
    /// May only be called once. Panics if called a second time.
    pub fn exception_handler(&self, handler: impl Fn(String) + Send + Sync + 'static) {
        assert!(self.exception_handler.set(Arc::new(handler)).is_ok(), "exception handler already set");
    }

    /// Loads a plugin of type `T` from the given native library path.
    ///
    /// The same type `T` may be loaded from multiple paths, yielding independent instances.
    /// Calling with the same `(T, path)` pair returns the previously loaded instance.
    ///
    /// # Errors
    /// Can fail if `path` was previously loaded for a different plugin type.
    pub fn load<T: PluginTrait + Send + Sync + 'static>(&self, lib_path: impl AsRef<Path>) -> Result<super::Plugin<T>, PluginLoadError> {
        let path = lib_path.as_ref().to_path_buf();

        {
            let inner = self.inner.lock().expect("runtime mutex poisoned");
            inner.plugins.check_uniqueness::<T>(&path)?;
            if let Some(arc) = inner.plugins.get_cached::<T>(&path) {
                return Ok(super::Plugin::new(arc));
            }
        }

        // Load the native library.
        let lib = unsafe { libloading::Library::new(path.as_os_str()) }.map_err(|e| PluginLoadError::LoadFailed(e.to_string()))?;

        let plugin = T::load_from(|symbol| {
            let symbol_bytes: Vec<u8> = symbol.bytes().chain(std::iter::once(0)).collect();
            match unsafe { lib.get::<extern "system" fn()>(&*symbol_bytes) } {
                Ok(f) => *f as *const u8,
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
            register_fn(TRAMPOLINE_UNCAUGHT_EXCEPTION, shared::uncaught_exception_callback as *const u8);
            register_fn(TRAMPOLINE_UNCAUGHT_EXCEPTION_CTX, ctx);
        }

        let arc = Arc::new(plugin);
        {
            let mut inner = self.inner.lock().expect("runtime mutex poisoned");
            inner.plugins.insert::<T>(path, Arc::clone(&arc));
            inner.libraries.push(lib);
        }
        Ok(super::Plugin::new(arc))
    }
}

static RUNTIME: OnceLock<AotRuntime> = OnceLock::new();

/// Returns the process-global AOT runtime.
///
/// This function lazily creates the singleton and returns a shared reference
/// on every subsequent call.
pub fn runtime() -> &'static AotRuntime {
    RUNTIME.get_or_init(AotRuntime::new)
}
