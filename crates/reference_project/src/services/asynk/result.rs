use crate::patterns::result::Error;
use interoptopus::pattern::asynk::Async;
use interoptopus::pattern::result::result_to_ffi;
use interoptopus::rt::Tokio;
use interoptopus::{AsyncRuntime, ffi};

#[ffi(service)]
#[derive(AsyncRuntime)]
pub struct ServiceAsyncResult {
    #[runtime]
    rt: Tokio,
}

#[ffi]
impl ServiceAsyncResult {
    pub fn create() -> ffi::Result<Self, Error> {
        result_to_ffi(|| {
            let rt = Tokio::new();
            Ok(Self { rt })
        })
    }

    pub async fn success(_: Async<Self>) -> ffi::Result<(), Error> {
        ffi::Result::Ok(())
    }

    pub async fn fail(_: Async<Self>) -> ffi::Result<(), Error> {
        ffi::Result::Err(Error::Fail)
    }
}
