use crate::patterns::result::FFIError;
use interoptopus::patterns::result::FFIResult;
use interoptopus::{ffi_service, ffi_type};

#[ffi_type(opaque)]
pub struct ServiceResult {}

#[ffi_service]
impl ServiceResult {
    pub fn new() -> FFIResult<Self, FFIError> {
        FFIResult::ok(Self {})
    }

    pub fn test(&self) -> FFIResult<(), FFIError> {
        FFIResult::err(FFIError::Fail)
    }
}
