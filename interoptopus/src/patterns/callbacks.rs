//! Useful when `extern "C" fn()` delegate types give compile errors.

use crate::lang::c::{CType, FnPointerType, FunctionSignature, Parameter};
use crate::lang::rust::CTypeInfo;

#[repr(transparent)]
pub struct CallbackXY<X1, R> {
    ptr: extern "C" fn(X1) -> R,
}

impl<X1, R> CallbackXY<X1, R> {
    pub fn call(&self, x1: X1) -> R {
        (self.ptr)(x1)
    }
}

impl<T1, R> CTypeInfo for CallbackXY<T1, R>
where
    T1: CTypeInfo,
    R: CTypeInfo,
{
    fn type_info() -> CType {
        let sig = FunctionSignature::new(vec![Parameter::new("x0".to_string(), T1::type_info())], R::type_info());
        CType::FnPointer(FnPointerType::new(sig))
    }
}
