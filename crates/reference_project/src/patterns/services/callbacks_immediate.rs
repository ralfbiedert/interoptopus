use crate::patterns::callbacks::{Callback, CallbackError};
use crate::patterns::result::{Error, FFIError};
use interoptopus::patterns::slice::FFISlice;
use interoptopus::{ffi_service, ffi_service_ctor, ffi_type};

/// Some struct we want to expose as a class.
#[ffi_type(opaque)]
pub struct ServiceCallbacksImmediate {}

// Regular implementation of methods.
#[ffi_service(error = "FFIError")]
impl ServiceCallbacksImmediate {
    #[ffi_service_ctor]
    pub fn new() -> Result<Self, Error> {
        Ok(Self {})
    }

    pub fn callback_simple(&mut self, callback: Callback) -> Result<(), Error> {
        callback.call(0);
        Ok(())
    }

    pub fn callback_ffi_return(&mut self, callback: CallbackError) -> Result<(), Error> {
        callback.call(0, 0);
        Ok(())
    }

    pub fn callback_with_slice(&mut self, callback: CallbackError, input: FFISlice<i32>) -> Result<(), Error> {
        callback.call(input.as_slice()[0], input.as_slice()[1]);
        Ok(())
    }
}
