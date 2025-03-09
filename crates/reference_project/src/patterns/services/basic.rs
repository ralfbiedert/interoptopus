use crate::patterns::result::FFIError;
use interoptopus::{ffi, ffi_service, ffi_type};

#[ffi_type(opaque)]
pub struct ServiceBasic {}

#[ffi_service]
impl ServiceBasic {
    pub fn new() -> ffi::Result<Self, FFIError> {
        ffi::Ok(Self {})
    }
}
