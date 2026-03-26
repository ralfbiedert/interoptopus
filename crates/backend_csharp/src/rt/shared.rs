//! Shared utilities used by both the dynamic (.NET CLR) and AOT (native library) runtimes.

use interoptopus::lang::plugin::PluginLoadError;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

/// Cache of loaded plugins, enforcing one-to-one mapping between plugin types and DLL paths.
pub struct PluginCache {
    plugins: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    type_to_path: HashMap<TypeId, PathBuf>,
    path_to_type: HashMap<PathBuf, TypeId>,
}

impl PluginCache {
    pub fn new() -> Self {
        Self { plugins: HashMap::new(), type_to_path: HashMap::new(), path_to_type: HashMap::new() }
    }

    /// Checks that the `(T, path)` combination doesn't conflict with existing entries.
    pub fn check_uniqueness<T: 'static>(&self, path: &PathBuf) -> Result<(), PluginLoadError> {
        let type_id = TypeId::of::<T>();

        if let Some(existing_path) = self.type_to_path.get(&type_id)
            && *existing_path != *path
        {
            return Err(PluginLoadError::LoadFailed(format!(
                "plugin {} already loaded from {}, cannot load from {}",
                std::any::type_name::<T>(),
                existing_path.display(),
                path.display()
            )));
        }
        if let Some(existing_type) = self.path_to_type.get(path)
            && *existing_type != type_id
        {
            return Err(PluginLoadError::LoadFailed(format!("DLL {} already loaded for a different plugin type", path.display())));
        }

        Ok(())
    }

    /// Returns a cached plugin instance if one exists for type `T`.
    pub fn get_cached<T: Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        let type_id = TypeId::of::<T>();
        self.plugins.get(&type_id).map(|boxed| {
            let arc = boxed.downcast_ref::<Arc<T>>().expect("type mismatch in plugin cache");
            Arc::clone(arc)
        })
    }

    /// Inserts a newly loaded plugin into the cache.
    pub fn insert<T: Send + Sync + 'static>(&mut self, path: PathBuf, arc: Arc<T>) {
        let type_id = TypeId::of::<T>();
        self.type_to_path.insert(type_id, path.clone());
        self.path_to_type.insert(path, type_id);
        self.plugins.insert(type_id, Box::new(arc));
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
