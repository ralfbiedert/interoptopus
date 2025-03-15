use crate::types::aliases::{FnPtrCharArray, FnPtru8u8};
use crate::types::arrays::CharArray;
use interoptopus::ffi_function;

#[ffi_function]
pub fn fnptr_1(callback: FnPtru8u8, value: u8) -> u8 {
    callback(value)
}

#[ffi_function]
pub fn fnptr_2(callback: FnPtrCharArray, value: CharArray) {
    callback(value)
}
