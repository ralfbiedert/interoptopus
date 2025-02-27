use crate::patterns::result::Error;
use crate::patterns::result::FFIError;
use interoptopus::patterns::asynk::{AsyncCallback, AsyncRuntime};
use interoptopus::patterns::result::FFIResult;
use interoptopus::{ffi_service, ffi_service_ctor, ffi_type};
use std::future::Future;
use std::sync::Arc;
use std::thread::{sleep, spawn};
use tokio::runtime::Runtime;

#[ffi_type(opaque)]
pub struct ServiceAsync {
    runtime: Runtime,
}

impl AsyncRuntime for ServiceAsync {
    fn spawn<F: Future<Output = ()> + Send + 'static>(&self, f: F) {
        self.runtime.spawn(f);
    }
}

#[ffi_service(error = "FFIError")]
impl ServiceAsync {
    #[ffi_service_ctor]
    pub fn new() -> Result<Self, Error> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .map_err(|_| Error::Bad)
            .unwrap();

        Ok(Self { runtime })
    }

    pub fn return_after_ms_explicit(&self, x: u64, ms: u64, async_callback: AsyncCallback<FFIResult<u64, FFIError>>) -> Result<(), FFIError> {
        spawn(move || {
            sleep(std::time::Duration::from_millis(ms));
            async_callback.call(&FFIResult::ok(x));
        });
        Ok(())
    }

    pub async fn return_after_ms(s: Arc<ServiceAsync>, x: u64, ms: u64) -> Result<u64, FFIError> {
        dbg!("I WORK!!!!!!");
        // sleep(std::time::Duration::from_millis(ms));
        // async_callback.call(&x);
        Ok(x)
    }
}
