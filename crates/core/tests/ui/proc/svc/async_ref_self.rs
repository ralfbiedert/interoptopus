use interoptopus::{ffi, pattern::asynk::AsyncRuntime};
use std::future::Future;

#[ffi]
enum Error {
    Something,
}

#[ffi(service)]
struct Service;

#[ffi]
impl Service {
    pub async fn bad(&self) -> ffi::Result<u8, Error> {
        ffi::Ok(0)
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
