use crate::patterns::result::Error;
use interoptopus::{ffi, ffi_service, ffi_type};

#[ffi_type(service)]
pub struct ServiceMain {
    val: u32,
}

#[ffi_service]
impl ServiceMain {
    pub fn new(x: u32) -> ffi::Result<Self, Error> {
        ffi::Ok(Self { val: x })
    }
}

#[ffi_type(service)]
pub struct ServiceDependent {
    val: u32,
}

#[ffi_service]
impl ServiceDependent {
    pub fn from_main(main: &ServiceMain) -> ffi::Result<Self, Error> {
        ffi::Ok(Self { val: main.val })
    }

    pub fn get(&self) -> u32 {
        self.val
    }
}
