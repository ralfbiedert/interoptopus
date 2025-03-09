use crate::patterns::result::Error;
use interoptopus::{ffi, ffi_service, ffi_type};

#[ffi_type(opaque)]
pub struct ServiceResult {}

#[ffi_service]
impl ServiceResult {
    pub fn new() -> ffi::Result<Self, Error> {
        ffi::Ok(Self {})
    }

    pub fn test(&self) -> ffi::Result<(), Error> {
        ffi::Err(Error::Fail)
    }
}
