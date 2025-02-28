use crate::patterns::result::{Error, FFIError};
use interoptopus::patterns::string::CStrPointer;
use interoptopus::{ffi_service, ffi_service_ctor, ffi_type};

/// Some struct we want to expose as a class.
#[ffi_type(opaque)]
pub struct ServiceMultipleCtors {
    pub data: Vec<u32>,
}

// Regular implementation of methods.
#[ffi_service(error = "FFIError")]
impl ServiceMultipleCtors {
    #[ffi_service_ctor]
    pub fn new_with(some_value: u32) -> Result<Self, Error> {
        Ok(Self { data: vec![some_value; some_value as usize] })
    }

    #[ffi_service_ctor]
    pub fn new_without() -> Result<Self, Error> {
        Ok(Self { data: vec![1, 2, 3] })
    }

    #[ffi_service_ctor]
    pub fn new_with_string(_: CStrPointer) -> Result<Self, Error> {
        Ok(Self { data: vec![1, 2, 3] })
    }

    #[ffi_service_ctor]
    pub fn new_failing(_some_value: u8) -> Result<Self, Error> {
        Err(Error::Bad)
    }
}
