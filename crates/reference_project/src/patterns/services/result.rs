use crate::patterns::result::FFIError;
use interoptopus::{ffi, ffi_service, ffi_type};

#[ffi_type(opaque)]
pub struct ServiceResult {}

#[ffi_service]
impl ServiceResult {
    pub fn new() -> ffi::Result<Self, FFIError> {
        ffi::Ok(Self {})
    }

    pub fn test(&self) -> ffi::Result<(), FFIError> {
        ffi::Err(FFIError::Fail)
    }
}
