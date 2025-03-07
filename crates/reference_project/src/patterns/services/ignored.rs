use crate::patterns::result::{Error, FFIError};
use interoptopus::patterns::result::FFIResult;
use interoptopus::{ffi_service, ffi_service_ctor, ffi_service_method, ffi_type};

#[ffi_type(opaque)]
pub struct ServiceIgnoringMethods {}

#[ffi_service]
impl ServiceIgnoringMethods {
    #[ffi_service_ctor]
    pub fn new() -> FFIResult<Self, FFIError> {
        FFIResult::ok(Self {})
    }

    #[ffi_service_method(ignore)]
    pub fn this_is_ignored(&mut self) -> Result<(), Error> {
        Ok(())
    }

    /// No FFI bindings are generated for non-pub methods.
    #[allow(unused)]
    fn not_exposed<T>(&mut self, _: T) -> Result<(), Error> {
        Ok(())
    }

    // Service methods without `self` are not valid for code generation and must be ignored.
    #[ffi_service_method(ignore)]
    pub fn test(_test: u32) -> Result<(), Error> {
        Ok(())
    }
}
