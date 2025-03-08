use crate::patterns::result::FFIError;
use interoptopus::patterns::result::FFIResult;
use interoptopus::patterns::string::CStrPointer;
use interoptopus::{ffi_service, ffi_type};

/// Some struct we want to expose as a class.
#[ffi_type(opaque)]
pub struct ServiceMultipleCtors {
    pub data: Vec<u32>,
}

// Regular implementation of methods.
#[ffi_service]
impl ServiceMultipleCtors {
    pub fn new_with(some_value: u32) -> FFIResult<Self, FFIError> {
        FFIResult::ok(Self { data: vec![some_value; some_value as usize] })
    }

    pub fn new_without() -> FFIResult<Self, FFIError> {
        FFIResult::ok(Self { data: vec![1, 2, 3] })
    }

    pub fn new_with_string(_: CStrPointer) -> FFIResult<Self, FFIError> {
        FFIResult::ok(Self { data: vec![1, 2, 3] })
    }

    pub fn new_failing(_some_value: u8) -> FFIResult<Self, FFIError> {
        FFIResult::err(FFIError::Fail)
    }
}
