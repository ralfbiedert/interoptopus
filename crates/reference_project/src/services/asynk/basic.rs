use crate::patterns::result::Error;
use interoptopus::ffi;
use interoptopus::pattern::asynk::{AsyncRuntime, AsyncSelf};
use interoptopus::pattern::result::result_to_ffi;
use interoptopus::{ffi_service, ffi_type};
use tokio::runtime::{Builder, Runtime};

type This = AsyncSelf<ServiceAsyncBasic>;

#[ffi_type(opaque)]
pub struct ServiceAsyncBasic {
    runtime: Runtime,
}

#[ffi_service]
impl ServiceAsyncBasic {
    pub fn new() -> ffi::Result<Self, Error> {
        result_to_ffi(|| {
            let runtime = Builder::new_multi_thread().build().map_err(|_| Error::Fail)?;
            Ok(Self { runtime })
        })
    }

    pub async fn call(_this: This) -> ffi::Result<(), Error> {
        ffi::Ok(())
    }
}

impl AsyncRuntime for ServiceAsyncBasic {
    fn spawn<Fn, F>(&self, f: Fn)
    where
        Fn: FnOnce(()) -> F,
        F: Future<Output = ()> + Send + 'static,
    {
        self.runtime.spawn(f(()));
    }
}
