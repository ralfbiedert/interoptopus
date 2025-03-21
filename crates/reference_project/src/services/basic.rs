use crate::patterns::result::ErrorREMOVEME;
use interoptopus::{ffi, ffi_service, ffi_type};

#[ffi_type(opaque)]
pub struct ServiceBasic {}

#[ffi_service]
impl ServiceBasic {
    pub fn new() -> ffi::Result<Self, ErrorREMOVEME> {
        ffi::Ok(Self {})
    }
}
