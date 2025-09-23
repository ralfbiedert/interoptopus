use interoptopus::{
    ffi, ffi_service, ffi_type,
    pattern::asynk::{Async, AsyncRuntime},
};
use std::future::Future;

#[ffi_type]
enum Error {
    Something,
}

#[ffi_type(service)]
struct Service;

#[ffi_service]
impl Service {
    pub async fn new(_: Async<Self>) -> ffi::Result<u8, Error> {
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

#[allow(dead_code)]
fn main() {}
