use crate::patterns::result::Error;
use interoptopus::ffi_type;
use interoptopus::pattern::asynk::{AsyncRuntime, AsyncSelf, AsyncThreadLocal};
use interoptopus::pattern::result::result_to_ffi;
use interoptopus::{ffi, ffi_service};
use tokio::runtime::{Builder, Runtime};

#[ffi_type(opaque)]
pub struct ServiceAsyncResult {
    runtime: Runtime,
}

#[ffi_service]
impl ServiceAsyncResult {
    pub fn new() -> ffi::Result<Self, Error> {
        result_to_ffi(|| {
            let runtime = Builder::new_multi_thread().build().map_err(|_| Error::Fail)?;
            Ok(Self { runtime })
        })
    }

    pub async fn success(_slf: AsyncThreadLocal<Self, ()>) -> ffi::Result<(), Error> {
        ffi::Result::Ok(())
    }

    pub async fn fail(_slf: AsyncSelf<Self>) -> ffi::Result<(), Error> {
        ffi::Result::Err(Error::Fail)
    }
}

impl AsyncRuntime for ServiceAsyncResult {
    fn spawn<Fn, F>(&self, f: Fn)
    where
        Fn: FnOnce(()) -> F,
        F: Future<Output = ()> + Send + 'static,
    {
        self.runtime.spawn(f(()));
    }
}
