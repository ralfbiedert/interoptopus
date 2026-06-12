//! Pre-built async runtimes for FFI services.
//!
//! Services that expose `async` methods need an [`AsyncRuntime`] implementor
//! to spawn futures. This module provides [`Tokio`], a ready-made
//! implementation backed by a multi-threaded Tokio runtime.
//!
//! For a different executor (e.g. single-threaded, or `async-std`),
//! [`AsyncRuntime`] can be implemented directly on a custom type instead.
//!
//! # Example
//!
//! A minimal async service with one async method:
//!
//! ```rust
//! # use interoptopus::{AsyncRuntime, ffi};
//! # use interoptopus::pattern::asynk::Async;
//! # use interoptopus::rt::Tokio;
//! #
//! # #[ffi]
//! # pub enum Error { Failed }
//! #
//! #[ffi(service)]
//! #[derive(AsyncRuntime)]
//! pub struct MyService {
//!     runtime: Tokio,
//! }
//!
//! #[ffi]
//! impl MyService {
//!     pub fn create() -> ffi::Result<Self, Error> {
//!         ffi::Ok(Self { runtime: Tokio::new() })
//!     }
//!
//!     pub async fn compute(_: Async<Self>, x: u32) -> ffi::Result<u32, Error> {
//!         ffi::Ok(x * 2)
//!     }
//! }
//! ```

use crate::pattern::asynk::{AsyncRuntime, TaskHandle};
use std::sync::{Arc, Mutex};

/// Inner wrapper that shuts down the Tokio runtime safely.
///
/// When a service struct owns its runtime and is wrapped in `Arc`, async tasks
/// may hold the last `Arc<Service>` reference. If C# disposes the service before
/// the task finishes cleaning up, the task's drop decrements the count to 0 — and
/// that drop happens on a Tokio worker thread. Calling `Runtime::drop()` from within
/// the runtime panics ("Cannot drop a runtime in a context where blocking is not
/// allowed"). This wrapper detects the async-context case and defers the shutdown to
/// a dedicated OS thread.
struct TokioInner {
    handle: tokio::runtime::Handle,
    rt: Mutex<Option<tokio::runtime::Runtime>>,
}

impl Drop for TokioInner {
    fn drop(&mut self) {
        let rt = self.rt.get_mut().unwrap_or_else(|e| e.into_inner()).take();
        let Some(rt) = rt else { return };
        if tokio::runtime::Handle::try_current().is_ok() {
            // We're inside a Tokio async context — blocking shutdown would deadlock.
            // Move the runtime to a fresh OS thread where blocking is allowed.
            std::thread::spawn(move || drop(rt));
        }
        // If not in async context, `rt` drops normally at end of scope (blocking OK).
    }
}

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
    rt: Arc<TokioInner>,
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
        let handle = rt.handle().clone();
        Self { rt: Arc::new(TokioInner { handle, rt: Mutex::new(Some(rt)) }) }
    }
}

impl AsyncRuntime for Tokio {
    type T = ();

    fn spawn<Fn, F>(&self, f: Fn) -> TaskHandle
    where
        Fn: FnOnce(Self::T) -> F + Send + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        let join = self.rt.handle.spawn(f(()));
        TaskHandle::from_handle(join.abort_handle(), tokio::task::AbortHandle::abort)
    }
}
