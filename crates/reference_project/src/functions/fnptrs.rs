use crate::types::aliases::{FnPtrCharArray, FnPtru8u8};
use crate::types::arrays::CharArray;
use interoptopus::ffi_function;

#[ffi_function]
pub fn fnptr_1(callback: FnPtru8u8, x: u8) -> u8 {
    callback(x)
}

#[ffi_function]
pub fn fnptr_2(callback: FnPtrCharArray, x: CharArray) {
    callback(x)
}
