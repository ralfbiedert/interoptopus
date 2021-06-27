use crate::types::{CallbackFFISlice, Vec3f32};
use interoptopus::ffi_function;
use interoptopus::patterns::callbacks::CallbackXY;
use interoptopus::patterns::slice::FFISlice;

static HUGE_VEC_SLICE: [Vec3f32; 100_000] = [Vec3f32 { x: 0.0, y: 0.0, z: 0.0 }; 100_000];

pub type CallbackHugeVecSlice<'a> = CallbackXY<FFISlice<'a, Vec3f32>, Vec3f32>;

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_ffi_slice_1(ffi_slice: FFISlice<u32>) -> u32 {
    ffi_slice.as_slice().unwrap_or(&[]).len() as u32
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_ffi_slice_2(ffi_slice: FFISlice<Vec3f32>, i: i32) -> Vec3f32 {
    ffi_slice.as_slice().map(|x| x.get(i as usize)).flatten().copied().unwrap_or(Vec3f32 {
        x: f32::NAN,
        y: f32::NAN,
        z: f32::NAN,
    })
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_ffi_slice_delegate(callback: CallbackFFISlice) -> u8 {
    callback.call(FFISlice::from_slice(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]))
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_ffi_slice_delegate_huge(callback: CallbackHugeVecSlice) -> Vec3f32 {
    callback.call(FFISlice::from_slice(&HUGE_VEC_SLICE))
}
