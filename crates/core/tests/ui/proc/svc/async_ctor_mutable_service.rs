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
struct Runtime;

#[ffi]
impl Runtime {
    pub fn create() -> ffi::Result<Self, Error> {
        ffi::Ok(Self)
    }
}

impl AsyncRuntime for Runtime {
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
    pub async fn create(_: Async<Runtime>) -> ffi::Result<Self, Error> {
        ffi::Ok(Self)
    }

    pub fn update(&mut self) {}
}

fn main() {}
