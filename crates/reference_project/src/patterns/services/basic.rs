use crate::patterns::result::FFIError;
use interoptopus::patterns::result::FFIResult;
use interoptopus::{ffi_service, ffi_type};

#[ffi_type(opaque)]
pub struct ServiceBasic {}

#[ffi_service]
impl ServiceBasic {
    pub fn new() -> FFIResult<Self, FFIError> {
        FFIResult::ok(Self {})
    }
}
