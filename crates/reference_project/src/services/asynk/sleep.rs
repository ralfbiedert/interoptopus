use crate::patterns::result::Error;
use interoptopus::pattern::asynk::Async;
use interoptopus::pattern::result::result_to_ffi;
use interoptopus::rt::Tokio;
use interoptopus::{AsyncRuntime, ffi};

#[ffi(service)]
#[derive(AsyncRuntime)]
pub struct ServiceAsyncSleep {
    runtime: Tokio,
}

#[ffi]
impl ServiceAsyncSleep {
    pub fn create() -> ffi::Result<Self, Error> {
        result_to_ffi(|| {
            let runtime = Tokio::new();
            Ok(Self { runtime })
        })
    }

    pub async fn return_after_ms(_: Async<Self>, x: u64, ms: u64) -> ffi::Result<u64, Error> {
        tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
        Ok(x).into()
    }
}
