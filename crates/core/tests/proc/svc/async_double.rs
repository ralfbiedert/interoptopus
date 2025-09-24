use interoptopus::{
    ffi,
    pattern::asynk::{Async, AsyncRuntime},
};
use std::future::Future;

#[ffi]
enum Error {
    Something,
}

#[ffi(service)]
struct Service;

#[ffi]
impl Service {
    pub fn new() -> ffi::Result<Self, Error> {
        ffi::Ok(Self)
    }

    pub async fn compute1(_: Async<Self>) -> ffi::Result<u8, Error> {
        ffi::Ok(0)
    }

    pub async fn compute2(_: Async<Self>) -> ffi::Result<u8, Error> {
        ffi::Ok(0)
    }
}

impl AsyncRuntime for Service {
    type T = ();

    fn spawn<Fn, F>(&self, _f: Fn)
    where
        Fn: FnOnce(()) -> F,
        F: Future<Output = ()> + Send + 'static,
    {
    }
}

fn main() {}
