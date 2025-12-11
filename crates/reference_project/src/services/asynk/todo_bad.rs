use crate::patterns::result::Error;
use interoptopus::ffi;
use interoptopus::AsyncRuntime;
use interoptopus::pattern::asynk::{AsyncRuntime, Async};
use interoptopus::pattern::result::result_to_ffi;
use interoptopus::rt::Tokio;
use interoptopus::{ffi_service, ffi_type};

#[derive(AsyncRuntime)]
#[ffi_type(opaque)]
pub struct ServiceAsyncTodoBad {
    runtime: Tokio,
}

#[ffi_service]
impl ServiceAsyncTodoBad {
    pub fn new() -> ffi::Result<Self, Error> {
        result_to_ffi(|| {
            Ok(Self { runtime: Tokio::new() })
        })
    }

    pub async fn call(_: Async<Self>) -> ffi::Result<(), Error> {
        ffi::Ok(())
    }

    // TODO: Once an `async fn` is present, methods accepting `&mut self` must not compile.
    pub fn bad(&mut self) {}
}
