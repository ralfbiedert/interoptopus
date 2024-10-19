use crate::patterns::callbacks::{MyCallback, SumDelegateReturn};
use crate::patterns::result::{Error, FFIError};
use interoptopus::patterns::primitives::FFIBool;
use interoptopus::patterns::slice::{FFISlice, FFISliceMut};
use interoptopus::patterns::string::CStrPointer;
use interoptopus::{ffi_service, ffi_service_ctor, ffi_service_ignore, ffi_service_method, ffi_type};
use std::ffi::CString;

/// Some struct we want to expose as a class.
#[ffi_type(opaque)]
#[derive(Default)]
pub struct SimpleService {
    pub some_value: u32,
    pub c_string: CString,
    pub data: Vec<u32>,
}

// Regular implementation of methods.
#[ffi_service(error = "FFIError")]
impl SimpleService {
    /// The constructor must return a `Result<Self, Error>`.
    #[ffi_service_ctor]
    pub fn new_with(some_value: u32) -> Result<Self, Error> {
        Ok(Self {
            some_value,
            c_string: CString::new("Hello new_with").unwrap(),
            data: vec![some_value; some_value as usize],
        })
    }

    #[ffi_service_ctor]
    pub fn new_without() -> Result<Self, Error> {
        Ok(Self {
            some_value: 0,
            c_string: CString::new("Hello new_without").unwrap(),
            data: vec![1, 2, 3],
        })
    }

    #[ffi_service_ctor]
    pub fn new_with_string(ascii: CStrPointer) -> Result<Self, Error> {
        Ok(Self {
            some_value: 0,
            c_string: ascii.as_c_str().unwrap().into(),
            data: vec![1, 2, 3],
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

    #[ffi_service_method(on_panic = "return_default")]
    pub fn method_value(&self, x: u32) -> u32 {
        x
    }

    /// This method should be documented.
    ///
    /// Multiple lines.
    #[ffi_service_method(on_panic = "return_default")]
    pub fn method_void(&self) {}

    /// Regular void functions don't need an annotation.
    pub fn method_void2(&self) {}

    #[ffi_service_method(on_panic = "return_default")]
    pub fn method_mut_self(&mut self, slice: FFISlice<u8>) -> u8 {
        *slice.as_slice().first().unwrap_or(&0)
    }

    /// Single line.
    #[ffi_service_method(on_panic = "return_default")]
    pub fn method_mut_self_void(&mut self, _slice: FFISlice<FFIBool>) {}

    #[ffi_service_method(on_panic = "return_default")]
    pub fn method_mut_self_ref(&mut self, x: &u8, _y: &mut u8) -> u8 {
        *x
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn method_mut_self_ref_slice(&mut self, x: &u8, _y: &mut u8, _slice: FFISlice<u8>) -> u8 {
        *x
    }

    #[ffi_service_method(on_panic = "return_default")]
    #[allow(clippy::needless_lifetimes)]
    pub fn method_mut_self_ref_slice_limited<'a, 'b>(&mut self, x: &u8, _y: &mut u8, _slice: FFISlice<'a, u8>, _slice2: FFISlice<'b, u8>) -> u8 {
        *x
    }

    // This annotation isn't really needed, `ffi_error` is standard error handling.
    #[ffi_service_method(on_panic = "ffi_error")]
    pub fn method_mut_self_ffi_error(&mut self, _slice: FFISliceMut<u8>) -> Result<(), Error> {
        Ok(())
    }

    pub fn method_mut_self_no_error(&mut self, mut slice: FFISliceMut<u8>) -> Result<(), Error> {
        slice.as_slice_mut();
        Ok(())
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

    /// This function has no panic safeguards. It will be a bit faster to
    /// call, but if it panics your host app will be in an undefined state.
    #[ffi_service_method(on_panic = "undefined_behavior")]
    pub fn return_string(&mut self) -> CStrPointer {
        CStrPointer::from_cstr(&self.c_string)
    }

    pub fn method_void_ffi_error(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[ffi_service_ignore]
    pub fn this_is_ignored(&mut self) -> Result<(), Error> {
        Ok(())
    }

    pub fn method_callback(&mut self, callback: MyCallback) -> Result<(), Error> {
        callback.call(0);
        Ok(())
    }

    pub fn method_callback_ffi_return(&mut self, callback: SumDelegateReturn) -> Result<(), Error> {
        callback.call(0, 0);
        Ok(())
    }

    pub fn method_callback_ffi_return_with_slice(&mut self, callback: SumDelegateReturn, input: FFISlice<i32>) -> Result<(), Error> {
        callback.call(input.as_slice()[0], input.as_slice()[1]);
        Ok(())
    }

    /// No FFI bindings are generated for non-pub methods.
    #[allow(unused)]
    fn not_exposed<T>(&mut self, _: T) -> Result<(), Error> {
        Ok(())
    }
}

// Some struct we want to expose as a class.
#[ffi_type(opaque)]
pub struct SimpleServiceLifetime<'a> {
    pub some_value: &'a u32,
}

#[ffi_service(error = "FFIError")]
impl<'a> SimpleServiceLifetime<'a> {
    #[ffi_service_ctor]
    pub fn new_with(some_value: &'a u32) -> Result<Self, Error> {
        Ok(Self { some_value })
    }

    pub fn method_lt(&mut self, _slice: FFISlice<'a, FFIBool>) {}

    pub fn method_lt2(&mut self, _slice: FFISlice<FFIBool>) {}

    // Sometimes lifetime params can get confused in low level codegen, so we have to replace `self` with explicit self.
    #[ffi_service_method(on_panic = "return_default")]
    pub fn return_string_accept_slice<'b>(_: &mut SimpleServiceLifetime<'b>, _: FFISlice<'b, u8>) -> CStrPointer<'b> {
        CStrPointer::empty()
    }

    pub fn method_void_ffi_error(&mut self) -> Result<(), Error> {
        Ok(())
    }
}
