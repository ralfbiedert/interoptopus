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
    pub async fn new(_: Async<Self>) -> ffi::Result<u8, Error> {
        ffi::Ok(0)
    }

    pub fn bad(&mut self) {}
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
