use crate::patterns::result::Error;
use interoptopus::pattern::asynk::{AsyncRuntime, AsyncSelf};
use interoptopus::{ffi, ffi_result};
use interoptopus::{ffi_service, ffi_type};
use tokio::runtime::{Builder, Runtime};

type This = AsyncSelf<ServiceAsyncBasic>;

#[ffi_type(opaque)]
pub struct ServiceAsyncBasic {
    runtime: Runtime,
}

#[ffi_service]
impl ServiceAsyncBasic {
    #[ffi_result]
    pub fn new() -> ffi::Result<Self, Error> {
        let runtime = Builder::new_multi_thread().build().map_err(|_| Error::Fail)?;
        Ok(Self { runtime })
    }

    #[ffi_result(asynk)]
    pub async fn call(_this: This) -> ffi::Result<(), Error> {
        Ok(())
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
