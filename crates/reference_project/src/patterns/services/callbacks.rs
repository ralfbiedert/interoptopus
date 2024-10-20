use crate::patterns::callbacks::{MyCallback, SumDelegateReturn};
use crate::patterns::result::{Error, FFIError};
use interoptopus::patterns::slice::FFISlice;
use interoptopus::{ffi_service, ffi_service_ctor, ffi_type};

/// Some struct we want to expose as a class.
#[ffi_type(opaque)]
pub struct ServiceCallbacks {}

// Regular implementation of methods.
#[ffi_service(error = "FFIError")]
impl ServiceCallbacks {
    #[ffi_service_ctor]
    pub fn new() -> Result<Self, Error> {
        Ok(Self {})
    }

    pub fn callback_simple(&mut self, callback: MyCallback) -> Result<(), Error> {
        callback.call(0);
        Ok(())
    }

    pub fn callback_ffi_return(&mut self, callback: SumDelegateReturn) -> Result<(), Error> {
        callback.call(0, 0);
        Ok(())
    }

    pub fn callback_with_slice(&mut self, callback: SumDelegateReturn, input: FFISlice<i32>) -> Result<(), Error> {
        callback.call(input.as_slice()[0], input.as_slice()[1]);
        Ok(())
    }
}
