use crate::patterns::success_enum::FFIError;
use interoptopus::{ffi_function, pattern_service_generated};
use some_rust_module::{Error, SimpleService};

mod some_rust_module {
    use interoptopus::ffi_type;

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
        pub fn new_with(some_value: u32) -> Self {
            Self { some_value }
        }

        pub fn method_result(&self, _: u32) -> Result<(), Error> {
            Ok(())
        }

        pub fn method_value(&self, x: u32) -> u32 {
            x
        }

        pub fn method_void(&self) {}

        pub fn method_mut_self(&mut self, x: u32) -> u32 {
            x
        }
    }
}

// Needed for Error to FFIError conversion.
impl From<Result<(), Error>> for FFIError {
    fn from(x: Result<(), Error>) -> Self {
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
        simple_service_mut_self(&mut SimpleService, x: u32) -> u32: method_mut_self,
        simple_service_void(&SimpleService) -> (): method_void
    ],
    [
        simple_service_extra_method
    ]
);
