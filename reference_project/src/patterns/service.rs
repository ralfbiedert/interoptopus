use crate::patterns::result::{Error, FFIError};
use interoptopus::patterns::primitives::FFIBool;
use interoptopus::patterns::slice::{FFISlice, FFISliceMut};
use interoptopus::patterns::string::AsciiPointer;
use interoptopus::{ffi_service, ffi_service_ctor, ffi_service_method, ffi_type};
use std::ffi::CString;

/// Some struct we want to expose as a class.
#[ffi_type(opaque)]
#[derive(Default)]
pub struct SimpleService {
    pub some_value: u32,
    pub c_string: CString,
}

// Regular implementation of methods.
#[ffi_service(error = "FFIError", prefix = "simple_service_")]
impl SimpleService {
    /// The constructor must return a `Result<Self, Error>`.
    #[ffi_service_ctor]
    pub fn new_with(some_value: u32) -> Result<Self, Error> {
        Ok(Self {
            some_value,
            c_string: CString::new("Hello new_with").unwrap(),
        })
    }

    #[ffi_service_ctor]
    pub fn new_without() -> Result<Self, Error> {
        Ok(Self {
            some_value: 0,
            c_string: CString::new("Hello new_without").unwrap(),
        })
    }

    #[ffi_service_ctor]
    pub fn new_failing(_some_value: u8) -> Result<Self, Error> {
        Err(Error::Bad)
    }

    /// Methods returning a Result<(), _> are the default and do not
    /// need annotations.
    pub fn method_result(&self, _: u32) -> Result<(), Error> {
        Ok(())
    }

    #[ffi_service_method(wrap = "direct")]
    pub fn method_value(&self, x: u32) -> u32 {
        x
    }

    /// This method should be documented.
    ///
    /// Multiple lines.
    #[ffi_service_method(wrap = "direct")]
    pub fn method_void(&self) {}

    #[ffi_service_method(wrap = "direct")]
    pub fn method_mut_self(&mut self, slice: FFISlice<u8>) -> u8 {
        *slice.as_slice().get(0).unwrap_or(&0)
    }

    /// Single line.
    #[ffi_service_method(wrap = "direct")]
    pub fn method_mut_self_void(&mut self, _slice: FFISlice<FFIBool>) {}

    #[ffi_service_method(wrap = "direct")]
    pub fn method_mut_self_ref(&mut self, x: &u8, _y: &mut u8) -> u8 {
        *x
    }

    #[ffi_service_method(wrap = "direct")]
    pub fn method_mut_self_ref_slice(&mut self, x: &u8, _y: &mut u8, _slice: FFISlice<u8>) -> u8 {
        *x
    }

    #[ffi_service_method(wrap = "direct")]
    pub fn method_mut_self_ref_slice_limited<'a, 'b>(&mut self, x: &u8, _y: &mut u8, _slice: FFISlice<'a, u8>, _slice2: FFISlice<'b, u8>) -> u8 {
        *x
    }

    pub fn method_mut_self_ffi_error(&mut self, _slice: FFISliceMut<u8>) -> Result<(), Error> {
        Ok(())
    }

    pub fn method_mut_self_no_error(&mut self, mut slice: FFISliceMut<u8>) -> Result<(), Error> {
        slice.as_slice_mut();
        Ok(())
    }

    #[ffi_service_method(wrap = "raw")]
    pub fn return_string(&mut self) -> AsciiPointer {
        AsciiPointer::from_cstr(&self.c_string)
    }

    pub fn method_void_ffi_error(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

// Some struct we want to expose as a class.
#[ffi_type(opaque)]
pub struct SimpleServiceLifetime<'a> {
    pub some_value: &'a u32,
}

#[ffi_service(error = "FFIError", prefix = "simple_service_lt_")]
impl<'a> SimpleServiceLifetime<'a> {
    #[ffi_service_ctor]
    pub fn new_with(some_value: &'a u32) -> Result<Self, Error> {
        Ok(Self { some_value })
    }

    #[ffi_service_method(wrap = "direct")]
    pub fn method_lt(&mut self, _slice: FFISlice<'a, FFIBool>) {}

    #[ffi_service_method(wrap = "direct")]
    pub fn method_lt2<'b>(&mut self, _slice: FFISlice<'b, FFIBool>) {}

    pub fn method_void_ffi_error(&mut self) -> Result<(), Error> {
        Ok(())
    }
}
