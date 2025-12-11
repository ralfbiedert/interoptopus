use crate::patterns::result::Error;
use interoptopus::AsyncRuntime;
use interoptopus::ffi_type;
use interoptopus::pattern::asynk::{Async, AsyncRuntime};
use interoptopus::pattern::result::result_to_ffi;
use interoptopus::rt::Tokio;
use interoptopus::{ffi, ffi_service};

#[derive(AsyncRuntime)]
#[ffi_type(opaque)]
pub struct ServiceAsyncResult {
    runtime: Tokio,
}

#[ffi_service]
impl ServiceAsyncResult {
    pub fn new() -> ffi::Result<Self, Error> {
        result_to_ffi(|| Ok(Self { runtime: Tokio::new() }))
    }

    pub async fn success(_: Async<Self>) -> ffi::Result<(), Error> {
        ffi::Result::Ok(())
    }

    pub async fn fail(_: Async<Self>) -> ffi::Result<(), Error> {
        ffi::Result::Err(Error::Fail)
    }
}
