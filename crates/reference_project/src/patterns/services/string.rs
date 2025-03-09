use crate::patterns::result::Error;
use interoptopus::{ffi, ffi_service, ffi_service_method, ffi_type};

/// Some struct we want to expose as a class.
#[ffi_type(opaque)]
pub struct ServiceStrings {}

// Regular implementation of methods.
#[ffi_service]
impl ServiceStrings {
    pub fn new() -> ffi::Result<Self, Error> {
        ffi::Result::ok(Self {})
    }

    pub fn pass_string(&mut self, _: ffi::CStrPointer) {}

    // If we actually return a value we have to declare what happens upon panic.
    #[ffi_service_method(on_panic = "abort")]
    pub fn return_string(&mut self) -> ffi::CStrPointer {
        ffi::CStrPointer::empty()
    }
}
