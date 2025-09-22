use crate::patterns::result::Error;
use crate::types::enums::EnumPayload;
use interoptopus::{ffi, ffi_service, ffi_type};

#[ffi_type(service)]
pub struct ServiceResult {}

#[ffi_service]
impl ServiceResult {
    pub fn new() -> ffi::Result<Self, Error> {
        ffi::Ok(Self {})
    }

    pub fn test(&self) -> ffi::Result<(), Error> {
        ffi::Err(Error::Fail)
    }

    pub fn result_u32(&self) -> ffi::Result<u32, Error> {
        ffi::Ok(123)
    }

    pub fn result_string(&self) -> ffi::Result<ffi::String, Error> {
        ffi::Ok(ffi::String::from("hello world".to_string()))
    }

    pub fn result_option_enum(&self) -> ffi::Result<ffi::Option<EnumPayload>, Error> {
        ffi::Ok(ffi::Some(EnumPayload::C(123)))
    }

    pub fn result_slice(&self, slice: ffi::Slice<u32>, i: u64) -> ffi::Result<u32, Error> {
        ffi::Ok(slice[i as usize])
    }
}
