use crate::patterns::result::Error;
use interoptopus::AsyncRuntime;
use interoptopus::ffi;
use interoptopus::pattern::asynk::{Async, AsyncRuntime};
use interoptopus::pattern::result::result_to_ffi;
use interoptopus::rt::Tokio;
use interoptopus::{ffi_service, ffi_type};

#[derive(AsyncRuntime)]
#[ffi_type(opaque)]
pub struct ServiceAsyncSleep {
    runtime: Tokio,
}

#[ffi_service]
impl ServiceAsyncSleep {
    pub fn new() -> ffi::Result<Self, Error> {
        result_to_ffi(|| Ok(Self { runtime: Tokio::new() }))
    }

    pub async fn return_after_ms(_: Async<Self>, x: u64, ms: u64) -> ffi::Result<u64, Error> {
        tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
        Ok(x).into()
    }
}
