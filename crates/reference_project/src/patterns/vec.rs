use interoptopus::{ffi, ffi_function};

#[ffi_function]
pub fn pattern_vec_1() -> ffi::Vec<u8> {
    vec![1, 2, 3].into()
}

#[ffi_function]
pub fn pattern_vec_2(_: ffi::Vec<u8>) {}

#[ffi_function]
pub fn pattern_vec_3(v: ffi::Vec<u8>) -> ffi::Vec<u8> {
    v
}

#[ffi_function]
pub fn pattern_vec_4(v: &ffi::Vec<u8>) -> ffi::Vec<u8> {
    v.clone()
}

/// TODO: This should be macro generated.
#[ffi_function]
pub fn interoptopus_vec_TODO_destroy(_: ffi::Vec<u8>) {}
