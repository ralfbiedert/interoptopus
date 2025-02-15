use crate::patterns::result::{Error, FFIError};
use interoptopus::patterns::string::CStrPointer;
use interoptopus::{ffi_service, ffi_service_ctor, ffi_service_method, ffi_type};
use std::ffi::CString;

/// Some struct we want to expose as a class.
#[ffi_type(opaque)]
pub struct ServiceOnPanic {
    pub c_string: CString,
}

// Regular implementation of methods.
#[ffi_service(error = "FFIError")]
impl ServiceOnPanic {
    #[ffi_service_ctor]
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            c_string: CString::new("Hello new_with").unwrap(),
        })
    }

    /// Methods returning a Result<(), _> are the default and do not
    /// need annotations.
    pub fn return_result(&self, _: u32) -> Result<(), Error> {
        Ok(())
    }

    /// Methods returning a value need an `on_panic` annotation.
    #[ffi_service_method(on_panic = "return_default")]
    pub fn return_default_value(&self, x: u32) -> u32 {
        x
    }

    /// This function has no panic safeguards. It will be a bit faster to
    /// call, but if it panics your host app will be in an undefined state.
    #[ffi_service_method(on_panic = "undefined_behavior")]
    pub fn return_ub_on_panic(&mut self) -> CStrPointer {
        CStrPointer::from_cstr(&self.c_string)
    }
}
