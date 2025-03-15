//! Transparent `async fn` support over FFI.

use crate::lang::TypeInfo;
use crate::lang::{Documentation, FnPointer, FunctionSignature, Meta, Parameter, Primitive, Type};
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

        let meta = Meta::with_documentation(Documentation::new());
        let sig = FunctionSignature::new(params, rval);
        let name = format!("AsyncCallback{}", T::type_info().name_within_lib());
        let fn_pointer = FnPointer::new_named(sig, name);
        let named_callback = pattern::AsyncCallback::with_meta(fn_pointer, meta);

        Type::Pattern(TypePattern::AsyncCallback(named_callback))
    }
}

/// Used as `this: AsyncSelf` instead of `self` when using `Send` runtimes.
pub struct AsyncSelf<S> {
    s: Arc<S>, // Self
}

impl<S> AsyncSelf<S> {
    pub fn new(s: Arc<S>) -> Self {
        Self { s }
    }
}

impl<S> Deref for AsyncSelf<S> {
    type Target = Arc<S>;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

/// Used as `this: AsyncThreadLocal` instead of `self` on `!Send` runtimes.
pub struct AsyncThreadLocal<S, T> {
    s: Arc<S>, // Self
    t: T,      // Thread locals from runtime
}

impl<S, T> AsyncThreadLocal<S, T> {
    pub fn new(s: Arc<S>, t: T) -> Self {
        Self { s, t }
    }

    pub fn slf(&self) -> &Arc<S> {
        &self.s
    }
}

impl<S, T> Deref for AsyncThreadLocal<S, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.t
    }
}

// /// Helper to produce a `AsyncCallback` and `AsyncThreadLocal` from proc macros.
#[doc(hidden)]
pub trait AsyncProxy<S, T> {
    fn new(s: Arc<S>, t: T) -> Self;
}

impl<S, T> AsyncProxy<S, T> for AsyncThreadLocal<S, T> {
    fn new(s: Arc<S>, t: T) -> Self {
        Self::new(s, t)
    }
}

impl<S, T> AsyncProxy<S, T> for AsyncSelf<S> {
    fn new(s: Arc<S>, _: T) -> Self {
        Self::new(s)
    }
}

/// Helper for async services using `Send` runtimes.
pub trait AsyncRuntime {
    fn spawn<Fn, F>(&self, f: Fn)
    where
        Fn: FnOnce(()) -> F,
        F: Future<Output = ()> + Send + 'static;
}

/// Helper for async services using `!Send` runtimes.
pub trait AsyncRuntimeThreadLocal {
    type ThreadLocal; // Thread local;

    fn spawn<Fn, F>(&self, f: Fn)
    where
        Fn: FnOnce(Self::ThreadLocal) -> F + Send + 'static,
        F: Future<Output = ()> + 'static;
}
