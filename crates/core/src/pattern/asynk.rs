//! Async method support for FFI services.
//!
//! Service methods marked `async fn` are automatically dispatched onto
//! the service's [`AsyncRuntime`]. Instead of `&self`, async methods
//! take [`Async<Self>`] as their first parameter — a thread-safe handle
//! that can be moved into the spawned future.
//!
//! The [`Async<S>`] wrapper dereferences to `Arc<S>`, giving shared
//! access to the service instance. An optional runtime context of type
//! [`AsyncRuntime::T`] is available via [`Async::context`].
//!
//! See the [`rt`](crate::rt) module for a ready-made Tokio-based runtime.
//!
//! # Example
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
//!     multiplier: u32,
//! }
//!
//! #[ffi]
//! impl MyService {
//!     pub fn create(multiplier: u32) -> ffi::Result<Self, Error> {
//!         ffi::Ok(Self { runtime: Tokio::new(), multiplier })
//!     }
//!
//!     /// Async methods take `Async<Self>` instead of `&self`.
//!     /// The wrapper dereferences to the service, so field access works normally.
//!     pub async fn compute(this: Async<Self>, x: u32) -> ffi::Result<u32, Error> {
//!         ffi::Ok(x * this.multiplier)
//!     }
//! }
//! ```
//!
//! # Why `Async<Self>` instead of `&self`?
//!
//! In a typical FFI call the foreign side (e.g. C#) calls into Rust, Rust
//! does its work synchronously, and control returns before any borrowed
//! pointers go out of scope. With async methods this model breaks: the
//! foreign caller invokes the method, but the actual work is spawned onto
//! a Rust async runtime and may complete long after the FFI call has
//! returned. The foreign side no longer governs the lifetime of the
//! operation — the Rust runtime does.
//!
//! This has two consequences:
//!
//! - **The service must be kept alive by shared ownership.** A borrowed
//!   `&self` would dangle once the FFI call returns, so `Async<Self>`
//!   wraps the service in an `Arc` that can be moved into the spawned
//!   future.
//!
//! - **Parameters must be owned.** Borrowed data (`&T`, slices, string
//!   references) cannot be used in async method signatures because there
//!   is no caller stack frame to anchor the borrow. All arguments must be
//!   types that own their data (e.g. `u32`, [`ffi::String`](crate::ffi::String),
//!   [`ffi::Vec<T>`](crate::ffi::Vec)).

use crate::bad_wire;
use crate::inventory::{Inventory, TypeId};
use crate::lang::meta::Visibility;
use crate::lang::types::{TypeInfo, TypeKind, TypePattern, WireIO};
use crate::wire::SerializationError;
use std::ffi::c_void;
use std::future::Future;
use std::io::{Read, Write};
use std::ops::Deref;
use std::pin::Pin;
use std::ptr::null;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

/// When used as the last parameter, makes a function `async`.
#[doc(hidden)]
#[repr(C)]
pub struct AsyncCallback<T>(Option<extern "C" fn(*const T, *const c_void) -> ()>, *const c_void);

// Manual Clone/Copy: the derive would add `T: Copy` / `T: Clone` bounds,
// but `T` only appears behind pointers in the struct fields so these impls
// are valid for any `T`.
impl<T> Clone for AsyncCallback<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for AsyncCallback<T> {}

// SAFETY: This is "safe-ish", as the type itself and its pointer are safe to send.
// However, this type must not be used / called with non-{send, sync} types. The proc
// macros generally make sure of that via static assertions, but user code doesn't.
unsafe impl<T> Send for AsyncCallback<T> {}
unsafe impl<T> Sync for AsyncCallback<T> {}

impl<T: TypeInfo> AsyncCallback<T> {
    ///   Creates a new instance of the callback using  `extern "C" fn`
    pub fn new(func: extern "C" fn(*const T, *const c_void)) -> Self {
        Self(Some(func), null())
    }

    /// Creates a callback with an explicit context pointer (e.g., a leaked `Arc` for use with [`AsyncCallbackFuture`]).
    pub fn with_context(func: extern "C" fn(*const T, *const c_void), context: *const c_void) -> Self {
        Self(Some(func), context)
    }

    /// Will call function if it exists, panic otherwise.
    ///
    /// # Safety
    ///
    /// `AsyncCallback` has blanket `Send` and `Sync` impls regardless of `T`.
    /// The caller must ensure that `T` is actually safe to send across threads,
    /// that the callback pointer and context are still valid, and that the
    /// pointee will not be used after this call (the callee takes ownership
    /// via `ptr::read`).
    pub unsafe fn call(&self, t: *const T) {
        self.0.expect("Assumed function would exist but it didn't.")(t, self.1);
    }

    /// Will call function only if it exists.
    ///
    /// # Safety
    ///
    /// `AsyncCallback` has blanket `Send` and `Sync` impls regardless of `T`.
    /// The caller must ensure that `T` is actually safe to send across threads,
    /// that the callback pointer and context are still valid, and that the
    /// pointee will not be used after this call (the callee takes ownership
    /// via `ptr::read`).
    pub unsafe fn call_if_some(&self, t: *const T) -> Option<()> {
        match self.0 {
            Some(c) => {
                c(t, self.1);
                Some(())
            }
            None => None,
        }
    }
}
impl<T: TypeInfo> From<extern "C" fn(*const T, *const c_void)> for AsyncCallback<T> {
    fn from(x: extern "C" fn(*const T, *const c_void) -> ()) -> Self {
        Self(Some(x), null())
    }
}

impl<T: TypeInfo> From<AsyncCallback<T>> for Option<extern "C" fn(*const T, *const c_void)> {
    fn from(x: AsyncCallback<T>) -> Self {
        x.0
    }
}

unsafe impl<T: TypeInfo> TypeInfo for AsyncCallback<T> {
    const WIRE_SAFE: bool = false;
    const RAW_SAFE: bool = T::RAW_SAFE;
    const ASYNC_SAFE: bool = T::ASYNC_SAFE;
    const SERVICE_SAFE: bool = false;
    const SERVICE_CTOR_SAFE: bool = false;

    fn id() -> TypeId {
        T::id().derive(0x3BA866E612BB2BEA769699B3476994B8)
    }

    fn kind() -> TypeKind {
        TypeKind::TypePattern(crate::lang::types::TypePattern::AsyncCallback(T::id()))
    }

    fn ty() -> crate::lang::types::Type {
        let t = T::ty();
        crate::lang::types::Type {
            emission: t.emission.clone(),
            docs: crate::lang::meta::Docs::empty(),
            visibility: Visibility::Public,
            name: format!("AsyncCallback<{}>", t.name),
            kind: Self::kind(),
        }
    }

    fn register(inventory: &mut impl Inventory) {
        // Ensure base type is registered.
        T::register(inventory);
        inventory.register_type(Self::id(), Self::ty());
    }
}

unsafe impl<T: WireIO> WireIO for AsyncCallback<T> {
    fn write(&self, _: &mut impl Write) -> Result<(), SerializationError> {
        bad_wire!()
    }

    fn read(_: &mut impl Read) -> Result<Self, SerializationError> {
        bad_wire!()
    }

    fn live_size(&self) -> usize {
        bad_wire!()
    }
}

/// Internal payload used by `AsyncCallbackFuture`.
struct FutureState<T> {
    result: Option<T>,
    waker: Option<Waker>,
    on_complete: Option<Box<dyn FnOnce() + Send + 'static>>,
}

extern "C" fn async_callback_complete<T: Send + 'static>(value: *const T, context: *const c_void) {
    // Safety: `context` is always an `Arc<Mutex<FutureState<T>>>` created in
    // `AsyncCallbackFuture::new` via `Arc::into_raw`. We reclaim ownership here —
    // this matches the one extra strong count deposited by `into_raw`.
    let state = unsafe { Arc::from_raw(context.cast::<Mutex<FutureState<T>>>()) };
    let mut lock = state.lock().unwrap();
    // Safety: The caller guarantees `value` is valid and that the pointee will not
    // be used afterwards (the caller forgets the original to prevent double-drop).
    lock.result = Some(unsafe { std::ptr::read(value) });
    if let Some(on_complete) = lock.on_complete.take() {
        on_complete();
    }
    if let Some(waker) = lock.waker.take() {
        waker.wake();
    }
}

/// A [`Future`] that resolves when its paired [`AsyncCallback<T>`] is invoked.
///
/// Use [`AsyncCallbackFuture::new`] to produce a matched `(future, callback)` pair.
/// Pass the callback to any FFI function accepting [`AsyncCallback<T>`], then
/// `.await` the future to receive the result.
///
/// # Lifetimes / cancellation
///
/// If the future is dropped before the callback fires, the shared state is kept
/// alive by the leaked Arc ref in the callback's context pointer and is freed
/// when the callback eventually fires. If the native side never calls the
/// callback the Arc leaks — this is the same contract as the underlying FFI.
pub struct AsyncCallbackFuture<T> {
    state: Arc<Mutex<FutureState<T>>>,
}

impl<T: Send + 'static + TypeInfo> AsyncCallbackFuture<T> {
    /// Creates a `(future, callback)` pair.
    pub fn new() -> (Self, AsyncCallback<T>) {
        let state = Arc::new(Mutex::new(FutureState { result: None, waker: None, on_complete: None }));
        let raw = Arc::into_raw(Arc::clone(&state)).cast::<c_void>();
        let cb = AsyncCallback::with_context(async_callback_complete::<T>, raw);
        (Self { state }, cb)
    }

    /// Creates a `(future, callback)` pair with a completion hook.
    ///
    /// `on_complete` is called inside the callback — i.e., at the moment the
    /// foreign side delivers the result — before the waiting future is woken.
    /// This measures true round-trip latency rather than executor scheduling latency.
    pub fn new_with_on_complete(on_complete: impl FnOnce() + Send + 'static) -> (Self, AsyncCallback<T>) {
        let state = Arc::new(Mutex::new(FutureState { result: None, waker: None, on_complete: Some(Box::new(on_complete)) }));
        let raw = Arc::into_raw(Arc::clone(&state)).cast::<c_void>();
        let cb = AsyncCallback::with_context(async_callback_complete::<T>, raw);
        (Self { state }, cb)
    }
}

impl<T: Send + 'static> Future for AsyncCallbackFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        let mut lock = self.state.lock().unwrap();
        if let Some(result) = lock.result.take() {
            Poll::Ready(result)
        } else {
            lock.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

/// Thread-safe handle to the service instance, used instead of `&self` in async methods.
///
/// Dereferences to `Arc<S>`, so service fields and methods are accessible
/// directly. An optional runtime-provided context is available via [`Self::context`].
pub struct Async<S: AsyncRuntime> {
    s: Arc<S>, // Self
    t: S::T,
}

impl<S: AsyncRuntime> Async<S> {
    pub fn new(s: Arc<S>, t: S::T) -> Self {
        Self { s, t }
    }

    pub fn context(&self) -> &S::T {
        &self.t
    }
}

impl<S: AsyncRuntime> Deref for Async<S> {
    type Target = Arc<S>;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

/// Executor for async service methods.
///
/// The associated type [`T`](Self::T) is a per-call context passed into the
/// spawned future and retrievable via [`Async::context`]. Use `()` if no
/// extra context is needed.
///
/// See the [`rt`](crate::rt) module for a ready-made Tokio implementation
/// or implement this trait directly for a custom executor.
pub trait AsyncRuntime {
    /// Per-call context handed to the spawned future.
    type T;

    /// Spawn a future onto the runtime, returning a handle that can abort it.
    fn spawn<Fn, F>(&self, f: Fn) -> TaskHandle
    where
        Fn: FnOnce(Self::T) -> F + Send + 'static,
        F: Future<Output = ()> + Send + 'static;
}

/// FFI-safe handle that can abort a spawned async task.
///
/// Returned by [`AsyncRuntime::spawn`]. The handle carries type-erased
/// function pointers so that any runtime can provide its own abort
/// mechanism without leaking implementation details through the FFI.
///
/// # C# usage
///
/// The generated C# code bridges `System.Threading.CancellationToken` to
/// this handle: when the C# token fires, it calls [`abort`](Self::abort),
/// which drops the Rust future at the next `.await` point. An
/// [`AsyncCallbackGuard`] inside the future ensures the completion
/// callback always fires (with a `Panic` result) so the C#
/// `TaskCompletionSource` is never leaked.
#[repr(C)]
pub struct TaskHandle {
    data: *mut (),
    abort_fn: Option<unsafe extern "C" fn(*mut ())>,
    drop_fn: Option<unsafe extern "C" fn(*mut ())>,
}

// SAFETY: The data pointer is opaque and only touched by the function pointers,
// which are safe to call from any thread. The runtime implementation is
// responsible for ensuring thread safety of the underlying abort mechanism.
unsafe impl Send for TaskHandle {}
unsafe impl Sync for TaskHandle {}

impl TaskHandle {
    /// Creates a task handle from any abort-able value.
    ///
    /// `handle` is the runtime-specific abort mechanism (e.g.
    /// [`tokio::task::AbortHandle`]). `abort` is called when
    /// cancellation is requested; `handle` is dropped when the
    /// `TaskHandle` itself is dropped.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let join = runtime.spawn(future);
    /// TaskHandle::from_handle(join.abort_handle(), tokio::task::AbortHandle::abort)
    /// ```
    pub fn from_handle<T: Send + 'static>(handle: T, abort: fn(&T)) -> Self {
        let boxed = Box::into_raw(Box::new(TaskHandleInner { handle, abort }));

        Self { data: boxed.cast(), abort_fn: Some(trampoline_abort::<T>), drop_fn: Some(trampoline_drop::<T>) }
    }

    /// Abort the task. The spawned future will be dropped at the next `.await` point.
    ///
    /// Calling this multiple times is safe — only the first call has effect.
    pub fn abort(&self) {
        if let Some(f) = self.abort_fn {
            unsafe {
                f(self.data);
            }
        }
    }

    /// Creates a handle that cannot abort anything.
    pub fn dummy() -> Self {
        Self { data: std::ptr::null_mut(), abort_fn: None, drop_fn: None }
    }
}

/// Typed payload stored behind the type-erased `data` pointer in [`TaskHandle`].
///
/// Created by [`TaskHandle::from_handle`], which heap-allocates this struct
/// via [`Box`] and stores the raw pointer as `*mut ()` in the handle.
struct TaskHandleInner<T> {
    handle: T,
    abort: fn(&T),
}

/// Type-erased abort trampoline stored in [`TaskHandle::abort_fn`].
///
/// Monomorphized for each concrete `T` by [`TaskHandle::from_handle`] so that
/// the correct type is recovered from the opaque pointer at call-time.
///
/// # Safety
///
/// `data` must point to a live, aligned `Box<TaskHandleInner<T>>` allocation
/// produced by [`TaskHandle::from_handle`]. This invariant is upheld by
/// construction: `from_handle` creates the allocation, and only
/// `trampoline_drop` frees it.
unsafe extern "C" fn trampoline_abort<T>(data: *mut ()) {
    // SAFETY: `data` was created by `Box::into_raw(Box::new(TaskHandleInner<T>))` in
    // `from_handle` and has not been freed yet (freeing only happens in `trampoline_drop`).
    // The cast back to the original type is valid because the same `T` that was used in
    // `from_handle` is baked into this monomorphized function at compile time.
    unsafe {
        let inner = &*(data.cast::<TaskHandleInner<T>>());
        (inner.abort)(&inner.handle);
    }
}

/// Type-erased drop trampoline stored in [`TaskHandle::drop_fn`].
///
/// Reclaims the heap allocation created by [`TaskHandle::from_handle`].
/// Called exactly once from [`TaskHandle::drop`].
///
/// # Safety
///
/// Same invariant as [`trampoline_abort`]: `data` must point to a live
/// `Box<TaskHandleInner<T>>` allocation. After this call the pointer is
/// invalid and must not be used again. [`TaskHandle::drop`] ensures
/// single-call by using [`Option::take`] on `drop_fn`.
unsafe extern "C" fn trampoline_drop<T>(data: *mut ()) {
    // SAFETY: `data` was created by `Box::into_raw` in `from_handle`.
    // `TaskHandle::drop` calls this exactly once (via `Option::take`),
    // so there is no double-free.
    unsafe {
        let _ = Box::from_raw(data.cast::<TaskHandleInner<T>>());
    }
}

impl Drop for TaskHandle {
    fn drop(&mut self) {
        if let Some(f) = self.drop_fn.take() {
            unsafe {
                f(self.data);
            }
        }
    }
}

unsafe impl TypeInfo for TaskHandle {
    const WIRE_SAFE: bool = false;
    const RAW_SAFE: bool = true;
    const ASYNC_SAFE: bool = false;
    const SERVICE_SAFE: bool = false;
    const SERVICE_CTOR_SAFE: bool = false;

    fn id() -> TypeId {
        TypeId::new(0xA4B3C2D1E0F98765_4321ABCDEF012345)
    }

    fn kind() -> TypeKind {
        TypeKind::TypePattern(TypePattern::TaskHandle)
    }

    fn ty() -> crate::lang::types::Type {
        crate::lang::types::Type {
            emission: crate::lang::meta::Emission::Builtin,
            docs: crate::lang::meta::Docs::empty(),
            visibility: Visibility::Public,
            name: "TaskHandle".to_string(),
            kind: Self::kind(),
        }
    }

    fn register(inventory: &mut impl Inventory) {
        inventory.register_type(Self::id(), Self::ty());
    }
}

unsafe impl WireIO for TaskHandle {
    fn write(&self, _: &mut impl Write) -> Result<(), SerializationError> {
        bad_wire!()
    }

    fn read(_: &mut impl Read) -> Result<Self, SerializationError> {
        bad_wire!()
    }

    fn live_size(&self) -> usize {
        bad_wire!()
    }
}

/// Drop guard ensuring an [`AsyncCallback`] is always invoked.
///
/// When a spawned future completes normally, call [`mark_completed`](Self::mark_completed)
/// before invoking the callback. If the future is aborted (e.g. via
/// [`TaskHandle::abort`]) the guard's [`Drop`] impl fires the stored
/// cancellation closure, which typically invokes the callback with
/// `ffi::Result::Panic` so the foreign side's task-completion mechanism
/// is never leaked.
///
/// # Thread safety
///
/// The `completed` flag uses [`AtomicBool`] with acquire/release ordering.
/// Because tokio only aborts futures at `.await` points, there is no race
/// between `mark_completed` (called in synchronous code after the last
/// `.await`) and `Drop` (called when the future is dropped). The atomic
/// is a defence-in-depth measure.
#[doc(hidden)]
pub struct AsyncCallbackGuard {
    completed: AtomicBool,
    on_cancel: Option<Box<dyn FnOnce() + Send>>,
}

impl AsyncCallbackGuard {
    /// Creates a guard that will call `on_cancel` if dropped before
    /// [`mark_completed`](Self::mark_completed) is called.
    pub fn new(on_cancel: impl FnOnce() + Send + 'static) -> Self {
        Self { completed: AtomicBool::new(false), on_cancel: Some(Box::new(on_cancel)) }
    }

    /// Mark the async operation as completed. Must be called before
    /// invoking the callback directly. Returns `true` if this was the
    /// first completion (i.e. the caller should proceed to fire the
    /// callback).
    pub fn mark_completed(&self) -> bool {
        !self.completed.swap(true, Ordering::AcqRel)
    }
}

impl Drop for AsyncCallbackGuard {
    fn drop(&mut self) {
        if !self.completed.swap(true, Ordering::AcqRel) {
            if let Some(f) = self.on_cancel.take() {
                f();
            }
        }
    }
}
