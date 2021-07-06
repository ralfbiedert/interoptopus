use crate::patterns::success_enum::FFIError;
use interoptopus::patterns::slice::FFISlice;
use interoptopus::{ffi_function, pattern_service_generated};
use some_rust_module::{Error, SimpleService};

pub mod some_rust_module {
    use crate::patterns::success_enum::FFIError;
    use interoptopus::ffi_type;
    use interoptopus::patterns::slice::FFISlice;

    // An error we use in a Rust library
    pub enum Error {
        Bad,
    }

    // Some struct we want to expose as a class.
    #[ffi_type(opaque)]
    #[derive(Default)]
    pub struct SimpleService {
        pub some_value: u32,
    }

    // Regular implementation of methods.
    impl SimpleService {
        pub fn new_with(some_value: u32) -> Result<Self, Error> {
            Ok(Self { some_value })
        }

        pub fn method_result(&self, _: u32) -> Result<(), Error> {
            Ok(())
        }

        pub fn method_value(&self, x: u32) -> u32 {
            x
        }

        pub fn method_void(&self) {}

        pub fn method_mut_self(&mut self, slice: FFISlice<u8>) -> u8 {
            *slice.as_slice().unwrap_or(&[0]).get(0).unwrap_or(&0)
        }

        pub fn method_mut_self_void(&mut self, _slice: FFISlice<u8>) {}

        pub fn method_mut_self_ffi_error(&mut self, _slice: FFISlice<u8>) -> FFIError {
            FFIError::Ok
        }
    }
}

// Needed for Error to FFIError conversion.
impl<T> From<Result<T, Error>> for FFIError {
    fn from(x: Result<T, Error>) -> Self {
        match x {
            Ok(_) => Self::Ok,
            Err(Error::Bad) => Self::Fail,
        }
    }
}

/// An extra exposed method.
#[ffi_function]
#[no_mangle]
pub extern "C" fn simple_service_extra_method(_context: Option<&mut SimpleService>) -> u32 {
    0
}

// Generate all FFI helpers.
pattern_service_generated!(
    simple_service_pattern,
    SimpleService,
    simple_service_create(x: u32) -> FFIError: new_with,
    simple_service_destroy() -> FFIError,
    [
        simple_service_result(&mut SimpleService, x: u32) -> FFIError: method_result,
        simple_service_value(&mut SimpleService, x: u32) -> u32: method_value,
        simple_service_mut_self(&mut SimpleService, slice: FFISlice<u8>) -> u8: method_mut_self,
        simple_service_mut_self_void(&mut SimpleService, slice: FFISlice<u8>) -> (): method_mut_self_void,
        simple_service_mut_self_ffi_error(&mut SimpleService, slice: FFISlice<u8>) -> FFIError: method_mut_self_ffi_error,
        simple_service_void(&SimpleService) -> (): method_void
    ],
    [
        simple_service_extra_method
    ]
);
