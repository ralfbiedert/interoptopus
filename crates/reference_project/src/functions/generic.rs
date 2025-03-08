use crate::types::{Generic, Generic2, Generic3, Generic4, Phantom, Weird1, Weird2};
use interoptopus::ffi_function;

#[ffi_function]
pub fn generic_1a(x: Generic<u32>, _y: Phantom<u8>) -> u32 {
    *x.x
}

#[ffi_function]
pub fn generic_1b(x: Generic<u8>, _y: Phantom<u8>) -> u8 {
    *x.x
}

#[ffi_function]
pub fn generic_1c<'a>(_x: Option<&'a Generic<'a, u8>>, y: &Generic<'a, u8>) -> u8 {
    *y.x
}

#[ffi_function]
pub fn generic_2(x: &Generic2<u8>) -> u8 {
    x.x
}

#[ffi_function]
pub fn generic_3(x: &Generic3<u8>) -> u8 {
    x.x
}

#[ffi_function]
pub fn generic_4(x: &Generic4<u8>) -> u8 {
    x.x
}

#[ffi_function]
pub fn generic_5(_x: Weird1<u32>, _y: Weird2<u8, 5>) -> bool {
    true
}
