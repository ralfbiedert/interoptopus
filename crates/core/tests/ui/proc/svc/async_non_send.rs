use interoptopus::{
    ffi,
    pattern::asynk::{Async, AsyncRuntime},
};
use std::future::Future;

#[ffi]
enum Error {
    Something,
}

/// A type that is not `Send`.
#[ffi(opaque)]
struct NonSendType {
    _data: *mut u8,
}

#[ffi(service)]
struct Service;

#[ffi]
impl Service {
    pub fn create() -> ffi::Result<Self, Error> {
        ffi::Ok(Self)
    }

    pub async fn compute(_: Async<Self>) -> ffi::Result<NonSendType, Error> {
        ffi::Ok(NonSendType { _data: std::ptr::null_mut() })
    }
}

impl AsyncRuntime for Service {
    type T = ();

    fn spawn<Fn, F>(&self, _f: Fn)
    where
        Fn: FnOnce(()) -> F + Send + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
    }
}

fn main() {}
