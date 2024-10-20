use crate::patterns::result::{Error, FFIError};
use interoptopus::patterns::primitives::FFIBool;
use interoptopus::patterns::slice::FFISlice;
use interoptopus::patterns::string::CStrPointer;
use interoptopus::{ffi_service, ffi_service_ctor, ffi_service_method, ffi_type};

/// Services can use lifetimes. However, they are more dangerous to use
/// via FFI, since you will not get any help tracking lifetimes there.
#[ffi_type(opaque)]
pub struct ServiceUsingLifetimes<'a> {
    pub some_value: &'a u32,
}

#[ffi_service(error = "FFIError")]
impl<'a> ServiceUsingLifetimes<'a> {
    #[ffi_service_ctor]
    pub fn new_with(some_value: &'a u32) -> Result<Self, Error> {
        Ok(Self { some_value })
    }

    pub fn lifetime_1(&mut self, _slice: FFISlice<'a, FFIBool>) {}

    pub fn lifetime_2(&mut self, _slice: FFISlice<FFIBool>) {}

    // Sometimes lifetime params can get confused in low level codegen, so we have to replace `self` with explicit self.
    #[ffi_service_method(on_panic = "return_default")]
    pub fn return_string_accept_slice<'b>(_: &mut ServiceUsingLifetimes<'b>, _: FFISlice<'b, u8>) -> CStrPointer<'b> {
        CStrPointer::empty()
    }
}
