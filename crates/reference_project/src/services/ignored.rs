use crate::patterns::result::Error;
use interoptopus::{ffi, ffi_service, ffi_type};

#[ffi_type(opaque)]
pub struct ServiceIgnoringMethods {}

#[ffi_service]
impl ServiceIgnoringMethods {
    pub fn new() -> ffi::Result<Self, Error> {
        ffi::Result::Ok(Self {})
    }

    // #[ffi_service_method(ignore)]
    pub fn this_is_ignored(&mut self) -> ffi::Result<(), Error> {
        ffi::Ok(())
    }

    /// No FFI bindings are generated for non-pub methods.
    #[allow(unused)]
    fn not_exposed<T>(&mut self, _: T) -> ffi::Result<(), Error> {
        ffi::Ok(())
    }

    // Service methods without `self` are not valid for code generation and must be ignored.
    // #[ffi_service_method(ignore)]
    pub fn test(&self, _: u32) -> ffi::Result<(), Error> {
        ffi::Ok(())
    }
}
