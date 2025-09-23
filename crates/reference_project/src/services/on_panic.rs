use crate::patterns::result::Error;
use interoptopus::{ffi, ffi_service, ffi_type};
use std::ffi::CString;

/// Some struct we want to expose as a class.
#[ffi_type(service)]
pub struct ServiceOnPanic {
    pub c_string: CString,
}

// Regular implementation of methods.
#[ffi_service]
impl ServiceOnPanic {
    pub fn new() -> ffi::Result<Self, Error> {
        ffi::Ok(Self { c_string: CString::new("Hello new_with").unwrap() })
    }

    /// Methods returning a Result<(), _> are the default and do not
    /// need annotations.
    pub fn return_result(&self, _: u32) -> ffi::Result<(), Error> {
        ffi::Ok(())
    }

    /// Methods returning a value need an `on_panic` annotation.
    pub fn return_default_value(&self, x: u32) -> u32 {
        x
    }

    /// This function has no panic safeguards. It will be a bit faster to
    /// call, but if it panics your host app will abort.
    pub fn return_ub_on_panic(&mut self) -> ffi::CStrPtr<'_> {
        ffi::CStrPtr::from_cstr(&self.c_string)
    }
}
