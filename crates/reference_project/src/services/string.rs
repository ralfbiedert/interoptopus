use crate::patterns::callback::StringCallback;
use crate::patterns::result::Error;
use interoptopus::{ffi, ffi_service, ffi_type};

/// Some struct we want to expose as a class.
#[ffi_type(opaque)]
pub struct ServiceStrings {}

// Regular implementation of methods.
#[ffi_service]
impl ServiceStrings {
    pub fn new() -> ffi::Result<Self, Error> {
        ffi::Result::Ok(Self {})
    }

    pub fn new_string(_x: ffi::String) -> ffi::Result<Self, Error> {
        ffi::Result::Ok(Self {})
    }

    pub fn pass_cstr(&mut self, _: ffi::CStrPtr) {}

    pub fn return_cstr(&mut self) -> ffi::CStrPtr {
        ffi::CStrPtr::empty()
    }

    pub fn callback_string(&self, s: ffi::String, cb: StringCallback) {
        cb.call(s.clone());
    }
}
