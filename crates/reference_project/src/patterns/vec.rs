use crate::types::basic::Vec3f32;
use crate::types::vec::UseSliceAndVec;
use interoptopus::ffi;

#[ffi]
pub fn pattern_vec_1() -> ffi::Vec<u8> {
    vec![1, 2, 3].into()
}

#[ffi]
pub fn pattern_vec_2(_: ffi::Vec<u8>) {}

#[ffi]
pub fn pattern_vec_3(v: ffi::Vec<u8>) -> ffi::Vec<u8> {
    v
}

#[ffi]
pub fn pattern_vec_4(v: &ffi::Vec<u8>) -> ffi::Vec<u8> {
    v.clone()
}

#[ffi]
pub fn pattern_vec_5(v: ffi::Vec<ffi::String>) -> ffi::Vec<ffi::String> {
    v
}

#[ffi]
pub fn pattern_vec_6(v: ffi::Vec<Vec3f32>) -> ffi::Vec<Vec3f32> {
    v
}

#[ffi]
pub fn pattern_vec_7(_: UseSliceAndVec) {}

#[ffi]
pub fn pattern_vec_8(v: UseSliceAndVec) -> UseSliceAndVec {
    v
}
