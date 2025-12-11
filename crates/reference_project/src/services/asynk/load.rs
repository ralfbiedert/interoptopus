use crate::patterns::result::Error;
use interoptopus::ffi;
use interoptopus::AsyncRuntime;
use interoptopus::pattern::asynk::{AsyncRuntime, Async};
use interoptopus::pattern::result::result_to_ffi;
use interoptopus::rt::Tokio;
use interoptopus::{ffi_service, ffi_type};

#[derive(AsyncRuntime)]
#[ffi_type(opaque)]
pub struct ServiceAsyncLoad {
    runtime: Tokio,
}

#[ffi_service]
impl ServiceAsyncLoad {
    pub fn new() -> ffi::Result<Self, Error> {
        result_to_ffi(|| {
            Ok(Self { runtime: Tokio::new() })
        })
    }

    pub async fn load(_: Async<Self>, x: u32) -> ffi::Result<u32, Error> {
        ffi::Ok(x)
    }
}