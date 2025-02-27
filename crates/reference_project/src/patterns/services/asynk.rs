use crate::patterns::result::Error;
use crate::patterns::result::FFIError;
use interoptopus::patterns::asynk::{AsyncCallback, AsyncRuntime};
use interoptopus::patterns::result::FFIResult;
use interoptopus::{ffi_service, ffi_service_ctor, ffi_type};
use std::future::Future;
use std::sync::Arc;
use std::thread::{sleep, spawn};

#[ffi_type(opaque)]
pub struct ServiceAsync {
    runtime: (),
}

impl AsyncRuntime for ServiceAsync {
    fn spawn<F: Future<Output = ()> + Send + 'static>(&self, f: F) {
        todo!()
    }
}

#[ffi_service(error = "FFIError", debug)]
impl ServiceAsync {
    #[ffi_service_ctor]
    pub fn new() -> Result<Self, Error> {
        Ok(Self { runtime: () })
    }

    pub async fn return_after_ms2(s: Arc<ServiceAsync>, x: u64, ms: u64) -> Result<u64, FFIError> {
        // sleep(std::time::Duration::from_millis(ms));
        // async_callback.call(&x);
        Ok(x)
    }

    pub fn return_after_ms(&self, x: u64, ms: u64, async_callback: AsyncCallback<FFIResult<u64, FFIError>>) -> Result<(), FFIError> {
        spawn(move || {
            sleep(std::time::Duration::from_millis(ms));
            async_callback.call(&FFIResult::ok(x));
        });
        Ok(())
    }
}
