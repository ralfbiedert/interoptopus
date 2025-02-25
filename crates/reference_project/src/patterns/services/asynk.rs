use crate::patterns::result::{Error, FFIError};
use interoptopus::patterns::asynk::AsyncCallback;
use interoptopus::patterns::result::FFIResult;
use interoptopus::{ffi_service, ffi_service_ctor, ffi_type};
use std::thread::{sleep, spawn};

#[ffi_type(opaque)]
pub struct ServiceAsync {
    runtime: (),
}

// impl Runtime for ServiceAsync {
//     pub spawn() {}
// }

#[ffi_service(error = "FFIError")]
impl ServiceAsync {
    #[ffi_service_ctor]
    pub fn new() -> Result<Self, Error> {
        Ok(Self { runtime: () })
    }

    // pub async fn return_after_ms2(&self, x: u64, ms: u64) -> Result<u64, FFIError> {
    //     // sleep(std::time::Duration::from_millis(ms));
    //     // async_callback.call(&x);
    //     Ok(x)
    // }

    pub fn return_after_ms(&self, x: u64, ms: u64, async_callback: AsyncCallback<FFIResult<u64, FFIError>>) -> Result<(), FFIError> {
        spawn(move || {
            sleep(std::time::Duration::from_millis(ms));
            async_callback.call(&FFIResult::ok(x));
        });
        Ok(())
    }

    // pub async fn do_work(&self) -> Result<u8, Error> {
    //     Ok(123)
    // }

    // pub fn do_work_raw(&self, callback: CallbackU8) -> FFIError {
    //     self.runtime.spawn(|| async {
    //         let rval = self.do_work().await;
    //         callback.call(rval);
    //     })
    // }
}

// trait RuntimeXXX {
//     fn spawn(&self, f: impl FnOnce() -> std::pin::Pin<Box<dyn std::future::Future<Output = ()>);
// }
//
// impl RuntimeXXX for AsyncService {
//
// }
