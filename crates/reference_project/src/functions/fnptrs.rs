use interoptopus::ffi;
use crate::types::aliases::{FnPtrCharArray, FnPtru8u8};
use crate::types::arrays::CharArray;

#[ffi]
pub fn fnptr_1(callback: FnPtru8u8, x: u8) -> u8 {
    callback(x)
}

#[ffi]
pub fn fnptr_2(callback: FnPtrCharArray, x: CharArray) {
    callback(x)
}
