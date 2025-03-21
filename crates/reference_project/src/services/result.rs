use crate::patterns::result::ErrorREMOVEME;
use interoptopus::{ffi, ffi_service, ffi_type};

#[ffi_type(opaque)]
pub struct ServiceResult {}

#[ffi_service]
impl ServiceResult {
    pub fn new() -> ffi::Result<Self, ErrorREMOVEME> {
        ffi::Ok(Self {})
    }

    pub fn test(&self) -> ffi::Result<(), ErrorREMOVEME> {
        ffi::Err(ErrorREMOVEME::Fail)
    }
}
