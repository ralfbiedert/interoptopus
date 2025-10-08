//! Transparent `async fn` support over FFI.

use crate::bad_wire;
use crate::inventory::{Inventory, TypeId};
use crate::lang::meta::Visibility;
use crate::lang::types::{SerializationError, TypeInfo, TypeKind, WireIO};
use std::ffi::c_void;
use std::future::Future;
use std::io::{Read, Write};
use std::ops::Deref;
use std::ptr::null;
use std::sync::Arc;

/// When used as the last parameter, makes a function `async`.
///
/// TODO: Document must be thread safe
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AsyncCallback<T>(Option<extern "C" fn(&T, *const c_void) -> ()>, *const c_void);

unsafe impl<T> Send for AsyncCallback<T> {}
unsafe impl<T> Sync for AsyncCallback<T> {}

impl<T: TypeInfo> AsyncCallback<T> {
    ///   Creates a new instance of the callback using  `extern "C" fn`
    pub fn new(func: extern "C" fn(&T, *const c_void)) -> Self {
        Self(Some(func), null())
    }

    ///   Will call function if it exists, panic otherwise.
    pub fn call(&self, t: &T) {
        self.0.expect("Assumed function would exist but it didn't.")(t, self.1);
    }

    ///   Will call function only if it exists
    pub fn call_if_some(&self, t: &T) -> Option<()> {
        match self.0 {
            Some(c) => {
                c(t, self.1);
                Some(())
            }
            None => None,
        }
    }
}
impl<T: TypeInfo> From<extern "C" fn(&T, *const c_void)> for AsyncCallback<T> {
    fn from(x: extern "C" fn(&T, *const c_void) -> ()) -> Self {
        Self(Some(x), null())
    }
}

impl<T: TypeInfo> From<AsyncCallback<T>> for Option<extern "C" fn(&T, *const c_void)> {
    fn from(x: AsyncCallback<T>) -> Self {
        x.0
    }
}

impl<T: TypeInfo> TypeInfo for AsyncCallback<T> {
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

    fn register(inventory: &mut Inventory) {
        // Ensure base type is registered.
        T::register(inventory);
        inventory.register_type(Self::id(), Self::ty());
    }
}

impl<T: WireIO> WireIO for AsyncCallback<T> {
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

/// Used as `this: AsyncSelf` instead of `self` when using `Send` runtimes.
/// TODO: Rust 1.91, emit const check that type_id of first async fn parameter equals Async<Service>
pub struct Async<S> {
    s: Arc<S>, // Self
}

impl<S> Async<S> {
    pub fn new(s: Arc<S>) -> Self {
        Self { s }
    }
}

impl<S> Deref for Async<S> {
    type Target = Arc<S>;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

/// Helper to produce a `AsyncCallback` and `AsyncThreadLocal` from proc macros.
#[doc(hidden)]
pub trait AsyncProxy<S, T> {
    fn new(s: Arc<S>, t: T) -> Self;
}

impl<S, T> AsyncProxy<S, T> for Async<S> {
    fn new(s: Arc<S>, _: T) -> Self {
        Self::new(s)
    }
}

/// Helper for async services using `Send` runtimes.
pub trait AsyncRuntime {
    type T;

    fn spawn<Fn, F>(&self, f: Fn)
    where
        Fn: FnOnce(Self::T) -> F,
        F: Future<Output = ()> + Send + 'static;
}
