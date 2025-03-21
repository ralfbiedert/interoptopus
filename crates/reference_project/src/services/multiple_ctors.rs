use crate::patterns::result::ErrorREMOVEME;
use interoptopus::{ffi, ffi_service, ffi_type};

/// Some struct we want to expose as a class.
#[ffi_type(opaque)]
pub struct ServiceMultipleCtors {
    pub data: Vec<u32>,
}

// Regular implementation of methods.
#[ffi_service]
impl ServiceMultipleCtors {
    pub fn new_with(some_value: u32) -> ffi::Result<Self, ErrorREMOVEME> {
        ffi::Ok(Self { data: vec![some_value; some_value as usize] })
    }

    pub fn new_without() -> ffi::Result<Self, ErrorREMOVEME> {
        ffi::Ok(Self { data: vec![1, 2, 3] })
    }

    pub fn new_with_string(_: ffi::CStrPointer) -> ffi::Result<Self, ErrorREMOVEME> {
        ffi::Ok(Self { data: vec![1, 2, 3] })
    }

    pub fn new_failing(_some_value: u8) -> ffi::Result<Self, ErrorREMOVEME> {
        ffi::Err(ErrorREMOVEME::Fail)
    }
}
