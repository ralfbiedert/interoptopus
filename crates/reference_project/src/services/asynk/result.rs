use crate::patterns::result::Error;
use interoptopus::ffi;
use interoptopus::pattern::asynk::{AsyncRuntime, AsyncSelf};
use interoptopus::pattern::result::result_to_ffi;
use interoptopus::{ffi_service, ffi_type};
use tokio::runtime::{Builder, Runtime};

type This = AsyncSelf<ServiceAsyncResult>;

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

    pub async fn success(_this: This) -> ffi::Result<(), Error> {
        ffi::Result::Ok(())
    }

    pub async fn fail(_this: This) -> ffi::Result<(), Error> {
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
