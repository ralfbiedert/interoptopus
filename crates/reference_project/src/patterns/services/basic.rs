use crate::patterns::result::{Error, FFIError};
use interoptopus::{ffi_service, ffi_service_ctor, ffi_type};

#[ffi_type(opaque)]
pub struct BasicService {}

#[ffi_service(error = "FFIError")]
impl BasicService {
    #[ffi_service_ctor]
    pub fn new() -> Result<Box<Self>, Error> {
        Ok(Box::new(Self {}))
    }
}
