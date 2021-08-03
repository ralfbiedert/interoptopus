use crate::patterns::result::FFIError;
use interoptopus::patterns::primitives::FFIBool;
use interoptopus::patterns::slice::{FFISlice, FFISliceMut};
use interoptopus::{ffi_service, ffi_service_ctor, ffi_service_method, ffi_type};
use std::fmt::{Display, Formatter};

// An error we use in a Rust library
#[derive(Debug)]
pub enum Error {
    Bad,
}

impl Display for Error {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl std::error::Error for Error {}

// Some struct we want to expose as a class.
#[ffi_type(opaque)]
#[derive(Default)]
pub struct SimpleService {
    pub some_value: u32,
}

// Regular implementation of methods.
#[ffi_service(error = "FFIError")]
impl SimpleService {
    /// The constructor must return a `Result<Self, Error>`.
    #[ffi_service_ctor]
    pub fn new_with(some_value: u32) -> Result<Self, Error> {
        Ok(Self { some_value })
    }

    /// Methods returning a Result<(), _> are the default and do not
    /// need annotations.
    pub fn method_result(&self, _: u32) -> Result<(), Error> {
        Ok(())
    }

    #[ffi_service_method(direct)]
    pub fn method_value(&self, x: u32) -> u32 {
        x
    }

    /// This method should be documented.
    ///
    /// Multiple lines.
    #[ffi_service_method(direct)]
    pub fn method_void(&self) {}

    #[ffi_service_method(direct)]
    pub fn method_mut_self(&mut self, slice: FFISlice<u8>) -> u8 {
        *slice.as_slice().get(0).unwrap_or(&0)
    }

    #[ffi_service_method(direct)]
    pub fn method_mut_self_void(&mut self, _slice: FFISlice<FFIBool>) {}

    #[ffi_service_method(direct)]
    pub fn method_mut_self_ref(&mut self, x: &u8, _y: &mut u8) -> u8 {
        *x
    }

    #[ffi_service_method(direct)]
    pub fn method_mut_self_ref_slice(&mut self, x: &u8, _y: &mut u8, _slice: FFISlice<u8>) -> u8 {
        *x
    }

    #[ffi_service_method(direct)]
    pub fn method_mut_self_ref_slice_limited<'a, 'b>(&mut self, x: &u8, _y: &mut u8, _slice: FFISlice<'a, u8>, _slice2: FFISlice<'b, u8>) -> u8 {
        *x
    }

    #[ffi_service_method(direct)]
    pub fn method_mut_self_ffi_error(&mut self, _slice: FFISliceMut<u8>) -> FFIError {
        FFIError::Ok
    }
}

impl From<Error> for FFIError {
    fn from(x: Error) -> Self {
        match x {
            Error::Bad => Self::Fail,
        }
    }
}
