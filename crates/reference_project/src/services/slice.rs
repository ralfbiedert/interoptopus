use crate::patterns::result::Error;
use interoptopus::{ffi, ffi_service, ffi_service_method, ffi_type};

/// Some struct we want to expose as a class.
#[ffi_type(opaque)]
pub struct ServiceVariousSlices {
    pub data: Vec<u32>,
}

// Regular implementation of methods.
#[ffi_service]
impl ServiceVariousSlices {
    pub fn new() -> ffi::Result<Self, Error> {
        ffi::Ok(Self { data: vec![123; 64] })
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn mut_self(&mut self, slice: ffi::Slice<u8>) -> u8 {
        *slice.as_slice().first().unwrap_or(&0)
    }

    /// Single line.
    #[ffi_service_method(on_panic = "return_default")]
    pub fn mut_self_void(&mut self, _slice: ffi::Slice<ffi::Bool>) {}

    #[ffi_service_method(on_panic = "return_default")]
    pub fn mut_self_ref(&mut self, x: &u8, _y: &mut u8) -> u8 {
        *x
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn mut_self_ref_slice(&mut self, x: &u8, _y: &mut u8, _slice: ffi::Slice<u8>) -> u8 {
        *x
    }

    #[ffi_service_method(on_panic = "return_default")]
    #[allow(clippy::needless_lifetimes)]
    pub fn mut_self_ref_slice_limited<'a, 'b>(&mut self, x: &u8, _y: &mut u8, _slice: ffi::Slice<'a, u8>, _slice2: ffi::Slice<'b, u8>) -> u8 {
        *x
    }

    // This annotation isn't really needed, `ffi_error` is standard error handling.
    #[ffi_service_method(on_panic = "ffi_error")]
    pub fn mut_self_ffi_error(&mut self, _slice: ffi::SliceMut<u8>) -> ffi::Result<(), Error> {
        ffi::Ok(())
    }

    pub fn mut_self_no_error(&mut self, mut slice: ffi::SliceMut<u8>) -> ffi::Result<(), Error> {
        slice.as_slice_mut();
        ffi::Ok(())
    }

    /// Warning, you _must_ discard the returned slice object before calling into this service
    /// again, as otherwise undefined behavior might happen.
    #[ffi_service_method(on_panic = "return_default")]
    pub fn return_slice(&mut self) -> ffi::Slice<'_, u32> {
        self.data.as_slice().into()
    }

    /// Warning, you _must_ discard the returned slice object before calling into this service
    /// again, as otherwise undefined behavior might happen.
    #[ffi_service_method(on_panic = "return_default")]
    pub fn return_slice_mut(&mut self) -> ffi::SliceMut<'_, u32> {
        ffi::SliceMut::from_slice(&mut self.data)
    }
}
