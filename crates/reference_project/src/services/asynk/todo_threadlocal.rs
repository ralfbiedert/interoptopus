use crate::patterns::result::Error;
use interoptopus::AsyncRuntime;
use interoptopus::ffi;
use interoptopus::pattern::asynk::{Async, AsyncRuntime};
use interoptopus::pattern::result::result_to_ffi;
use interoptopus::rt::Tokio;
use interoptopus::{ffi_service, ffi_type};

pub struct ThreadLocal {
    _x: u32,
}

#[derive(AsyncRuntime)]
#[ffi_type(opaque)]
pub struct ServiceAsyncThreadLocal {
    runtime: Tokio,
}

#[ffi_service]
impl ServiceAsyncThreadLocal {
    pub fn new() -> ffi::Result<Self, Error> {
        result_to_ffi(|| Ok(Self { runtime: Tokio::new() }))
    }

    pub async fn call_async_self(_: Async<Self>) -> ffi::Result<(), Error> {
        ffi::Ok(())
    }

    pub async fn call_async_thread_local(_: Async<Self>) -> ffi::Result<(), Error> {
        ffi::Ok(())
    }
}
