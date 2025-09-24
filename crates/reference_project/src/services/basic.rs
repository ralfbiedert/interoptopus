use crate::patterns::result::Error;
use interoptopus::ffi;

#[ffi(service)]
pub struct ServiceBasic {}

#[ffi]
impl ServiceBasic {
    pub fn new() -> ffi::Result<Self, Error> {
        ffi::Ok(Self {})
    }
}
