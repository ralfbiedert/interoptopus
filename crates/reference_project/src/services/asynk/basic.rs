use crate::patterns::result::Error;
use interoptopus::pattern::asynk::Async;
use interoptopus::pattern::result::result_to_ffi;
use interoptopus::rt::Tokio;
use interoptopus::{ffi, AsyncRuntime};

#[ffi(service)]
#[derive(AsyncRuntime)]
pub struct ServiceAsyncBasic {
    runtime: Tokio,
}

#[ffi]
impl ServiceAsyncBasic {
    pub fn new() -> ffi::Result<Self, Error> {
        result_to_ffi(|| {
            let runtime = Tokio::new();
            Ok(Self { runtime })
        })
    }

    pub async fn call(_: Async<Self>) -> ffi::Result<(), Error> {
        ffi::Ok(())
    }
}
