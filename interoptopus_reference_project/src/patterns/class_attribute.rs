use crate::types::FFIError;
use interoptopus::patterns::class::ClassPattern;
use interoptopus::{ffi_class, ffi_function, ffi_type};

pub enum Error {
    Bad,
}

impl From<Error> for FFIError {
    fn from(_: Error) -> Self {
        Self::Fail
    }
}

#[ffi_type(opaque)]
pub struct Context2 {
    xxx: u32,
}

impl ClassPattern for Context2 {
    type Result = Result<(), Self::Error>;
    type Error = Error;
    type FFIError = FFIError;

    fn null_error() -> Self::Error {
        Self::Error::Bad
    }

    fn new() -> Result<Self, Self::Error> {
        Ok(Self { xxx: 0 })
    }
}

#[ffi_class]
impl Context2 {
    fn method1(&self, x: u32) -> <Self as ClassPattern>::Result {
        Ok(())
    }

    // fn method2(&self, x: u32) -> u32 {
    //     1
    // }
    fn method2(&self, x: u32) -> <Self as ClassPattern>::Result {
        Ok(())
    }
}
