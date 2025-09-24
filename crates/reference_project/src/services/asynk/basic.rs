use crate::patterns::result::Error;
use interoptopus::ffi;
use interoptopus::pattern::asynk::{Async, AsyncRuntime};
use interoptopus::pattern::result::result_to_ffi;
use tokio::runtime::{Builder, Runtime};

#[ffi(service)]
pub struct ServiceAsyncBasic {
    runtime: Runtime,
}

#[ffi]
impl ServiceAsyncBasic {
    pub fn new() -> ffi::Result<Self, Error> {
        result_to_ffi(|| {
            let runtime = Builder::new_multi_thread().build().map_err(|_| Error::Fail)?;
            Ok(Self { runtime })
        })
    }

    pub async fn call(_: Async<Self>) -> ffi::Result<(), Error> {
        ffi::Ok(())
    }
}

impl AsyncRuntime for ServiceAsyncBasic {
    type T = ();

    fn spawn<Fn, F>(&self, f: Fn)
    where
        Fn: FnOnce(()) -> F,
        F: Future<Output = ()> + Send + 'static,
    {
        self.runtime.spawn(f(()));
    }
}
