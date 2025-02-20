use crate::patterns::result::{Error, FFIError};
use interoptopus::{ffi_service, ffi_service_ctor, ffi_type};
use crate::patterns::slice::CallbackU8;


#[ffi_type(opaque)]
pub struct AsyncService {
    runtime: (),
}


#[ffi_service(error = "FFIError")]
impl AsyncService {
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
