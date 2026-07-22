use interoptopus::{
    ffi,
    pattern::asynk::{Async, AsyncRuntime, TaskHandle},
};
use std::future::Future;

#[ffi]
enum Error {
    Something,
}

#[ffi(service)]
struct MutableRuntime;

#[ffi]
impl MutableRuntime {
    pub fn create() -> ffi::Result<Self, Error> {
        ffi::Ok(Self)
    }

    pub fn update(&mut self) {}
}

impl AsyncRuntime for MutableRuntime {
    type T = ();

    fn spawn<Fn, F>(&self, _f: Fn) -> TaskHandle
    where
        Fn: FnOnce(()) -> F + Send + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        TaskHandle::dummy()
    }
}

#[ffi(service)]
struct Service;

#[ffi]
impl Service {
    pub async fn create(_: Async<MutableRuntime>) -> ffi::Result<Self, Error> {
        ffi::Ok(Self)
    }
}

fn main() {}
