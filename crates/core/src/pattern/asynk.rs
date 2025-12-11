//! Transparent `async fn` support over FFI.

use crate::lang::TypeInfo;
use crate::lang::{Docs, FnPointer, Meta, Parameter, Primitive, Signature, Type};
use crate::pattern;
use crate::pattern::TypePattern;
use std::ffi::c_void;
use std::future::Future;
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

unsafe impl<T: TypeInfo> TypeInfo for AsyncCallback<T> {
    fn type_info() -> Type {
        let rval = <() as TypeInfo>::type_info();

        let params = vec![
            Parameter::new("value_ptr".to_string(), Type::ReadPointer(Box::new(T::type_info()))),
            Parameter::new("callback_data".to_string(), Type::ReadPointer(Box::new(Type::Primitive(Primitive::Void)))),
        ];

        let meta = Meta::with_docs(Docs::new());
        let sig = Signature::new(params, rval);
        let name = format!("AsyncCallback{}", T::type_info().name_within_lib());
        let fn_pointer = FnPointer::new_named(sig, name);
        let named_callback = pattern::AsyncCallback::with_meta(fn_pointer, meta);

        Type::Pattern(TypePattern::AsyncCallback(named_callback))
    }
}

/// Used as `this: AsyncSelf` instead of `self` when using `Send` runtimes.
/// TODO: Rust 1.91, emit const check that `type_id` of first async fn parameter equals Async<Service>
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

/// Helper for async services using `Send` runtimes.
pub trait AsyncRuntime {
    type T;

    fn spawn<Fn, F>(&self, f: Fn)
    where
        Fn: FnOnce(Self::T) -> F,
        F: Future<Output = ()> + Send + 'static;
}
