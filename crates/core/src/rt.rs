//! Optional runtime support for async services.

use crate::pattern::asynk::AsyncRuntime;
use std::sync::Arc;

/// A ready-made [`AsyncRuntime`] backed by a multi-threaded Tokio runtime.
///
/// Use this as the runtime field in async service structs. It creates a
/// multi-threaded Tokio runtime with all features enabled on construction.
///
/// # Example
///
/// ```rust
/// use interoptopus::{AsyncRuntime, ffi};
/// use interoptopus::rt::Tokio;
///
/// #[ffi(service)]
/// #[derive(AsyncRuntime)]
/// pub struct MyAsyncService {
///     runtime: Tokio,
/// }
/// ```
#[derive(Clone)]
pub struct Tokio {
    rt: Arc<tokio::runtime::Runtime>,
}

impl Default for Tokio {
    fn default() -> Self {
        Self::new()
    }
}

impl Tokio {
    #[must_use]
    pub fn new() -> Self {
        let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
        Self { rt: Arc::new(rt) }
    }
}

impl AsyncRuntime for Tokio {
    type T = ();

    fn spawn<Fn, F>(&self, f: Fn)
    where
        Fn: FnOnce(Self::T) -> F,
        F: Future<Output = ()> + Send + 'static,
    {
        self.rt.spawn(f(()));
    }
}
