use crate::patterns::result::Error;
use interoptopus::ffi;
use interoptopus::pattern::asynk::AsyncRuntime;
use interoptopus::pattern::result::result_to_ffi;
use interoptopus::{ffi_service, ffi_type};
use tokio::runtime::{Builder, Runtime};

#[ffi_type(opaque)]
pub struct ServiceAsyncTodoBad {
    runtime: Runtime,
}

#[ffi_service]
impl ServiceAsyncTodoBad {
    pub fn new() -> ffi::Result<Self, Error> {
        result_to_ffi(|| {
            let runtime = Builder::new_multi_thread().build().map_err(|_| Error::Fail)?;
            Ok(Self { runtime })
        })
    }

    // TODO: This must not compile.
    pub fn bad(&mut self) {}
}

impl AsyncRuntime for ServiceAsyncTodoBad {
    fn spawn<Fn, F>(&self, f: Fn)
    where
        Fn: FnOnce(()) -> F,
        F: Future<Output = ()> + Send + 'static,
    {
        self.runtime.spawn(f(()));
    }
}
