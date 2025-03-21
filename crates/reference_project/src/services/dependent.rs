use crate::patterns::result::ErrorREMOVEME;
use interoptopus::{ffi, ffi_service, ffi_type};

#[ffi_type(opaque)]
pub struct ServiceMain {
    value: u32,
}

#[ffi_service]
impl ServiceMain {
    pub fn new(value: u32) -> ffi::Result<Self, ErrorREMOVEME> {
        ffi::Ok(Self { value })
    }
}

#[ffi_type(opaque)]
pub struct ServiceDependent {
    value: u32,
}

#[ffi_service]
impl ServiceDependent {
    pub fn from_main(main: &ServiceMain) -> ffi::Result<Self, ErrorREMOVEME> {
        ffi::Ok(Self { value: main.value })
    }

    pub fn get(&self) -> u32 {
        self.value
    }
}
