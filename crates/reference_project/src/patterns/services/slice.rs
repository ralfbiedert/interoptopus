use crate::patterns::result::FFIError;
use interoptopus::patterns::primitives::FFIBool;
use interoptopus::patterns::result::FFIResult;
use interoptopus::patterns::slice::{FFISlice, FFISliceMut};
use interoptopus::{ffi_service, ffi_service_method, ffi_type};

/// Some struct we want to expose as a class.
#[ffi_type(opaque)]
pub struct ServiceVariousSlices {
    pub data: Vec<u32>,
}

// Regular implementation of methods.
#[ffi_service]
impl ServiceVariousSlices {
    pub fn new() -> FFIResult<Self, FFIError> {
        FFIResult::ok(Self { data: vec![123; 64] })
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn mut_self(&mut self, slice: FFISlice<u8>) -> u8 {
        *slice.as_slice().first().unwrap_or(&0)
    }

    /// Single line.
    #[ffi_service_method(on_panic = "return_default")]
    pub fn mut_self_void(&mut self, _slice: FFISlice<FFIBool>) {}

    #[ffi_service_method(on_panic = "return_default")]
    pub fn mut_self_ref(&mut self, x: &u8, _y: &mut u8) -> u8 {
        *x
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn mut_self_ref_slice(&mut self, x: &u8, _y: &mut u8, _slice: FFISlice<u8>) -> u8 {
        *x
    }

    #[ffi_service_method(on_panic = "return_default")]
    #[allow(clippy::needless_lifetimes)]
    pub fn mut_self_ref_slice_limited<'a, 'b>(&mut self, x: &u8, _y: &mut u8, _slice: FFISlice<'a, u8>, _slice2: FFISlice<'b, u8>) -> u8 {
        *x
    }

    // This annotation isn't really needed, `ffi_error` is standard error handling.
    #[ffi_service_method(on_panic = "ffi_error")]
    pub fn mut_self_ffi_error(&mut self, _slice: FFISliceMut<u8>) -> FFIResult<(), FFIError> {
        FFIResult::ok(())
    }

    pub fn mut_self_no_error(&mut self, mut slice: FFISliceMut<u8>) -> FFIResult<(), FFIError> {
        slice.as_slice_mut();
        FFIResult::ok(())
    }

    /// Warning, you _must_ discard the returned slice object before calling into this service
    /// again, as otherwise undefined behavior might happen.
    #[ffi_service_method(on_panic = "return_default")]
    pub fn return_slice(&mut self) -> FFISlice<u32> {
        self.data.as_slice().into()
    }

    /// Warning, you _must_ discard the returned slice object before calling into this service
    /// again, as otherwise undefined behavior might happen.
    #[ffi_service_method(on_panic = "return_default")]
    pub fn return_slice_mut(&mut self) -> FFISliceMut<u32> {
        FFISliceMut::from_slice(&mut self.data)
    }
}
