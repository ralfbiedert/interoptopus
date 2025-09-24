use crate::patterns::callback::StringCallback;
use crate::patterns::result::Error;
use interoptopus::ffi;

/// Some struct we want to expose as a class.
#[ffi(service)]
pub struct ServiceStrings {}

// Regular implementation of methods.
#[ffi]
impl ServiceStrings {
    pub fn new() -> ffi::Result<Self, Error> {
        ffi::Result::Ok(Self {})
    }

    pub fn new_string(_x: ffi::String) -> ffi::Result<Self, Error> {
        ffi::Result::Ok(Self {})
    }

    pub fn pass_cstr(&mut self, _: ffi::CStrPtr) {}

    pub fn return_cstr(&mut self) -> ffi::CStrPtr<'_> {
        ffi::CStrPtr::empty()
    }

    pub fn callback_string(&self, s: ffi::String, cb: StringCallback) {
        cb.call(s.clone());
    }
}
