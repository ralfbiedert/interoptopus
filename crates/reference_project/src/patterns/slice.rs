use crate::types::arrays::CharArray;
use crate::types::basic::Vec3f32;
use interoptopus::pattern::slice::{Slice, SliceMut};
use interoptopus::{callback, ffi, ffi_function};

static HUGE_VEC_SLICE: [Vec3f32; 100_000] = [Vec3f32 { x: 0.0, y: 0.0, z: 0.0 }; 100_000];

callback!(CallbackHugeVecSlice(slice: Slice<Vec3f32>) -> Vec3f32);
callback!(CallbackSliceMut(slice: SliceMut<'_, u8>) -> ());
callback!(CallbackU8(value: u8) -> u8);
callback!(CallbackCharArray2(value: CharArray) -> ());
callback!(CallbackFFISlice(slice: ffi::Slice<u8>) -> u8);

#[ffi_function]
pub fn pattern_ffi_slice_1(ffi_slice: Slice<u32>) -> u32 {
    ffi_slice.as_slice().len() as u32
}

#[ffi_function]
pub fn pattern_ffi_slice_1b(ffi_slice: SliceMut<u32>) -> u32 {
    ffi_slice.as_slice().len() as u32
}

#[ffi_function]
pub fn pattern_ffi_slice_2(ffi_slice: Slice<Vec3f32>, i: i32) -> Vec3f32 {
    ffi_slice
        .as_slice()
        .get(i as usize)
        .copied()
        .unwrap_or(Vec3f32 { x: f32::NAN, y: f32::NAN, z: f32::NAN })
}

#[ffi_function]
pub fn pattern_ffi_slice_delegate(callback: CallbackFFISlice) -> u8 {
    callback.call(Slice::from_slice(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]))
}

#[ffi_function]
pub fn pattern_ffi_slice_delegate_huge(callback: CallbackHugeVecSlice) -> Vec3f32 {
    callback.call(Slice::from_slice(&HUGE_VEC_SLICE))
}

#[ffi_function]
pub fn pattern_ffi_slice_3(mut slice: SliceMut<u8>, callback: CallbackSliceMut) {
    if let [x, ..] = slice.as_slice_mut() {
        *x += 1;
    }
    callback.call(slice);
}

#[ffi_function]
pub fn pattern_ffi_slice_4(_slice: Slice<u8>, _slice2: SliceMut<u8>) {}

/// It is (probably?) UB to call this function with the same FFI slice data at the same time.
#[ffi_function]
pub fn pattern_ffi_slice_5(slice: &Slice<u8>, slice2: &mut SliceMut<u8>) {
    let _ = slice.as_slice().len();
    let _ = slice2.as_slice().len();
}

#[ffi_function]
pub fn pattern_ffi_slice_6(slice: &SliceMut<u8>, callback: CallbackU8) {
    callback.call(slice.as_slice().first().copied().unwrap_or(0));
}

// UNSUPPORTED FOR NOW - Unclear how to handle string slices in C# with LibraryImport
// #[ffi_function]
// pub fn pattern_ffi_slice_7(slices: FFISliceMut<CStrPointer>) -> u32 {
//     let mut sum = 0;
//
//     for s in slices.as_slice() {
//         if let Ok(s) = s.as_str() {
//             sum += s.chars().count() as u32;
//         }
//     }
//
//     sum
// }

#[ffi_function]
pub fn pattern_ffi_slice_8(slice: &SliceMut<CharArray>, callback: CallbackCharArray2) {
    callback.call(slice.as_slice().first().copied().unwrap());
}
