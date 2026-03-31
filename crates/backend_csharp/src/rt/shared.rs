//! Shared utilities used by both the dynamic (.NET CLR) and AOT (native library) runtimes.

use interoptopus::lang::plugin::PluginLoadError;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Cache of loaded plugins, keyed by `(TypeId, path)`.
///
/// The same plugin type may be loaded from multiple DLL paths, yielding independent instances.
/// A single DLL path may only be used for one plugin type.
pub struct PluginCache {
    plugins: HashMap<(TypeId, PathBuf), Box<dyn Any + Send + Sync>>,
    path_to_type: HashMap<PathBuf, TypeId>,
}

impl PluginCache {
    pub fn new() -> Self {
        Self { plugins: HashMap::new(), path_to_type: HashMap::new() }
    }

    /// Checks that `path` has not already been loaded for a different plugin type.
    pub fn check_uniqueness<T: 'static>(&self, path: &Path) -> Result<(), PluginLoadError> {
        let type_id = TypeId::of::<T>();
        if let Some(existing_type) = self.path_to_type.get(path)
            && *existing_type != type_id
        {
            return Err(PluginLoadError::LoadFailed(format!("DLL {} already loaded for a different plugin type", path.display())));
        }
        Ok(())
    }

    /// Returns a cached plugin instance if `T` was already loaded from `path`.
    pub fn get_cached<T: Send + Sync + 'static>(&self, path: &Path) -> Option<Arc<T>> {
        let key = (TypeId::of::<T>(), path.to_path_buf());
        self.plugins.get(&key).map(|boxed| {
            let arc = boxed.downcast_ref::<Arc<T>>().expect("type mismatch in plugin cache");
            Arc::clone(arc)
        })
    }

    /// Inserts a newly loaded plugin into the cache.
    pub fn insert<T: Send + Sync + 'static>(&mut self, path: PathBuf, arc: Arc<T>) {
        let type_id = TypeId::of::<T>();
        self.path_to_type.insert(path.clone(), type_id);
        self.plugins.insert((type_id, path), Box::new(arc));
    }
}

/// Concrete `Sized` wrapper so we can store the trait-object handler behind a thin pointer.
pub struct HandlerShim {
    pub handler: Arc<dyn Fn(String) + Send + Sync>,
}

/// Callback registered with the managed plugin for uncaught exceptions.
#[allow(clippy::cast_ptr_alignment)]
pub unsafe extern "C" fn uncaught_exception_callback(ctx: *const u8, message: *const u8, len: i32) {
    let shim = unsafe { &*ctx.cast::<HandlerShim>() };
    let bytes = unsafe { std::slice::from_raw_parts(message, len.unsigned_abs() as usize) };
    let msg = String::from_utf8_lossy(bytes).into_owned();
    (shim.handler)(msg);
}
