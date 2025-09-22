use crate::patterns::result::Error;
use interoptopus::{ffi, ffi_service, ffi_type};

#[ffi_type(service)]
pub struct ServiceMain {
    value: u32,
}

#[ffi_service]
impl ServiceMain {
    pub fn new(x: u32) -> ffi::Result<Self, Error> {
        ffi::Ok(Self { value: x })
    }
}

#[ffi_type(opaque)]
pub struct ServiceDependent {
    value: u32,
}

#[ffi_service]
impl ServiceDependent {
    pub fn from_main(main: &ServiceMain) -> ffi::Result<Self, Error> {
        ffi::Ok(Self { value: main.value })
    }

    pub fn get(&self) -> u32 {
        self.value
    }
}
