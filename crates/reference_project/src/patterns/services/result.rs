use crate::patterns::result::FFIError;
use interoptopus::patterns::result::FFIResult;
use interoptopus::{ffi_service, ffi_service_ctor, ffi_type};

#[ffi_type(opaque)]
pub struct ServiceResult {}

#[ffi_service]
impl ServiceResult {
    #[ffi_service_ctor]
    pub fn new() -> FFIResult<Self, FFIError> {
        FFIResult::ok(Self {})
    }

    pub fn test() -> FFIResult<(), FFIError> {
        FFIResult::err(FFIError::Fail)
    }
}
