use crate::patterns::result::{Error, FFIError};
use interoptopus::{ffi_function, ffi_service, ffi_service_ctor, ffi_type};
use interoptopus::patterns::asynk::AsyncCallback;


#[ffi_function]
pub fn __async_mock(x: u64, async_callback: AsyncCallback<u64>)  {
    async_callback.call(&x);
}


#[ffi_type(opaque)]
pub struct ServiceAsync {
    runtime: (),
}


#[ffi_service(error = "FFIError")]
impl ServiceAsync {
    #[ffi_service_ctor]
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            runtime: ()
        })
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
