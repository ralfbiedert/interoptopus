use crate::patterns::result::Error;
use interoptopus::ffi;
use interoptopus::pattern::asynk::{AsyncRuntime, AsyncSelf};
use interoptopus::pattern::result::{result_to_ffi, result_to_ffi_async};
use interoptopus::{ffi_service, ffi_type};
use tokio::runtime::{Builder, Runtime};

#[ffi_type(opaque)]
pub struct ServiceAsyncSleep {
    runtime: Runtime,
}

#[ffi_service]
impl ServiceAsyncSleep {
    pub fn new() -> ffi::Result<Self, Error> {
        result_to_ffi(|| {
            let runtime = Builder::new_multi_thread().enable_time().build().map_err(|_| Error::Fail)?;
            Ok(Self { runtime })
        })
    }

    pub async fn return_after_ms(_: AsyncSelf<Self>, x: u64, ms: u64) -> ffi::Result<u64, Error> {
        result_to_ffi_async(async || {
            tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
            Ok(x)
        })
        .await
    }
}

impl AsyncRuntime for ServiceAsyncSleep {
    fn spawn<Fn, F>(&self, f: Fn)
    where
        Fn: FnOnce(()) -> F,
        F: Future<Output = ()> + Send + 'static,
    {
        self.runtime.spawn(f(()));
    }
}
