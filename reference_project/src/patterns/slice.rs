use crate::types::{CallbackFFISlice, Vec3f32};
use interoptopus::patterns::slice::{FFISlice, FFISliceMut};
use interoptopus::{callback, ffi_function};

static HUGE_VEC_SLICE: [Vec3f32; 100_000] = [Vec3f32 { x: 0.0, y: 0.0, z: 0.0 }; 100_000];

callback!(CallbackHugeVecSlice(slice: FFISlice<Vec3f32>) -> Vec3f32);
callback!(CallbackSliceMut(slice: FFISliceMut<'_, u8>) -> ());
callback!(CallbackU8(value: u8) -> u8);

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_ffi_slice_1(ffi_slice: FFISlice<u32>) -> u32 {
    ffi_slice.as_slice().len() as u32
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_ffi_slice_2(ffi_slice: FFISlice<Vec3f32>, i: i32) -> Vec3f32 {
    ffi_slice.as_slice().get(i as usize).copied().unwrap_or(Vec3f32 {
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

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_ffi_slice_3(mut slice: FFISliceMut<u8>, callback: CallbackSliceMut) {
    if let [x, ..] = slice.as_slice_mut() {
        *x += 1;
    }

    callback.call(slice);
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_ffi_slice_4(_slice: FFISlice<u8>, _slice2: FFISliceMut<u8>) {}

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_ffi_slice_5(slice: &FFISlice<u8>, slice2: &mut FFISliceMut<u8>) {
    let _ = slice.as_slice().len();
    let _ = slice2.as_slice().len();
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_ffi_slice_6(slice: &FFISliceMut<u8>, callback: CallbackU8) {
    callback.call(slice.as_slice().get(0).copied().unwrap_or(0));
}

// Some extra tests that were hard to do from core crate.
#[cfg(test)]
mod test {
    use crate::patterns::slice::pattern_ffi_slice_3;
    use interoptopus::patterns::slice::FFISliceMut;

    #[allow(dead_code)]
    extern "C" fn f(mut x: FFISliceMut<u8>) {
        let slice = x.as_slice_mut();
        slice[1] = 100;
    }

    #[test]
    fn test_pattern_ffi_slice_3<'a>() {
        let mut data = [0, 1, 2, 3, 4, 5];

        let jfc: extern "C" fn(FFISliceMut<'_, u8>) -> () = f;

        pattern_ffi_slice_3(FFISliceMut::from_slice(&mut data), jfc.into());

        assert_eq!(&data, &[1, 100, 2, 3, 4, 5])
    }
}
