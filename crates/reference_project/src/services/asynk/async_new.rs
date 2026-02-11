use crate::patterns::result::Error;
use interoptopus::AsyncRuntime;
use interoptopus::ffi;
use interoptopus::ffi_async_constructor;
use interoptopus::pattern::asynk::{Async, AsyncRuntime};
use interoptopus::pattern::result::result_to_ffi;
use interoptopus::rt::Tokio;
use interoptopus::{ffi_service, ffi_type};

#[derive(AsyncRuntime)]
#[ffi_type(opaque)]
pub struct ServiceAsyncNew {
    runtime: Tokio,
}

#[ffi_service]
impl ServiceAsyncNew {
    #[ffi_async_constructor]
    pub async fn new(wrapper: Async<Wrapper>) -> ffi::Result<Self, Error> {
        result_to_ffi(|| Ok(Self { runtime: wrapper.runtime.clone() }))
    }

    pub async fn call(_: Async<Self>) -> ffi::Result<(), Error> {
        ffi::Ok(())
    }
}

#[derive(AsyncRuntime)]
#[ffi_type(opaque)]
pub struct Wrapper {
    pub runtime: Tokio,
}

#[ffi_service]
impl Wrapper {
    pub fn new() -> ffi::Result<Self, Error> {
        result_to_ffi(|| Ok(Self { runtime: Tokio::new() }))
    }
}
