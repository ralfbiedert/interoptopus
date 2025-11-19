use crate::patterns::result::Error;
use interoptopus::pattern::asynk::{Async, AsyncRuntime};
use interoptopus::pattern::result::result_to_ffi;
use interoptopus::rt::Tokio;
use interoptopus::{ffi, AsyncRuntime};

#[ffi(service)]
#[derive(AsyncRuntime)]
pub struct ServiceAsyncResult {
    #[runtime(forward)]
    runtime: Tokio,
}

#[ffi]
impl ServiceAsyncResult {
    pub fn new() -> ffi::Result<Self, Error> {
        result_to_ffi(|| {
            let runtime = Tokio::new();
            Ok(Self { runtime })
        })
    }

    pub async fn success(_: Async<Self>) -> ffi::Result<(), Error> {
        ffi::Result::Ok(())
    }

    pub async fn fail(_: Async<Self>) -> ffi::Result<(), Error> {
        ffi::Result::Err(Error::Fail)
    }
}
