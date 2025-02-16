//! Functions using all supported type patterns.

use crate::patterns::result::{Error, FFIError};
use crate::types::{
    ambiguous1, ambiguous2, common, Array, BoolField, CallbackCharArray, Callbacku8u8, CharArray, EnumDocumented, EnumRenamedXYZ, FixedString, Generic, Generic2,
    Generic3, Generic4, NestedArray, Packed1, Packed2, Phantom, StructDocumented, StructRenamedXYZ, Transparent, Tupled, Vec3f32, Visibility1, Visibility2, Weird1,
    Weird2,
};
use interoptopus::patterns::option::FFIOption;
use interoptopus::patterns::result::panics_and_errors_to_ffi_enum;
use interoptopus::patterns::slice::FFISlice;
use interoptopus::patterns::slice::FFISliceMut;
use interoptopus::{ffi_function, here};
use std::time::Duration;

#[ffi_function]
pub fn primitive_void() {}

#[ffi_function]
#[allow(clippy::unused_unit)]
pub fn primitive_void2() -> () {}

#[ffi_function]
pub fn primitive_bool(x: bool) -> bool {
    !x
}

#[ffi_function]
pub fn primitive_u8(x: u8) -> u8 {
    u8::MAX - x
}

#[ffi_function]
pub fn primitive_u16(x: u16) -> u16 {
    u16::MAX - x
}

#[ffi_function]
pub fn primitive_u32(x: u32) -> u32 {
    u32::MAX - x
}

#[ffi_function]
pub fn primitive_u64(x: u64) -> u64 {
    u64::MAX - x
}

#[ffi_function]
pub fn primitive_i8(x: i8) -> i8 {
    -x
}

#[ffi_function]
pub fn primitive_i16(x: i16) -> i16 {
    -x
}

#[ffi_function]
pub fn primitive_i32(x: i32) -> i32 {
    -x
}

#[ffi_function]
pub fn primitive_i64(x: i64) -> i64 {
    -x
}

// UNSUPPORTED FOR NOW - Unclear how to handle BooleanAlignment in C# with LibraryImport
// #[ffi_function]
// pub fn boolean_alignment(mut x: BooleanAlignment) -> BooleanAlignment {
//     x.is_valid = !x.is_valid;
//     x
// }
//
// #[ffi_function]
// pub fn boolean_alignment2(rval: bool) -> BooleanAlignment {
//     BooleanAlignment {
//         is_valid: rval,
//         ..Default::default()
//     }
// }

// #[ffi_function]
// pub fn aligned_to_packed1(a: Aligned1) -> Packed1 {
//     dbg!(a.x);
//     dbg!(a.y);
//     Packed1 { x: a.x, y: a.y }
// }
//
// #[ffi_function(debug)]
// pub fn aligned_to_packed2(a: Aligned2) -> Packed2 {
//     Packed2 { x: a.x, y: a.y }
// }

#[ffi_function]
pub fn packed_to_packed1(a: Packed1) -> Packed2 {
    Packed2 { x: a.x, y: a.y }
}

#[ffi_function]
pub fn many_args_5(x0: i64, x1: i64, x2: i64, x3: i64, x4: i64) -> i64 {
    x0 + x1 + x2 + x3 + x4
}

#[ffi_function]
pub fn many_args_10(x0: i64, x1: i64, x2: i64, x3: i64, x4: i64, x5: i64, x6: i64, x7: i64, x8: i64, x9: i64) -> i64 {
    x0 + x1 + x2 + x3 + x4 + x5 + x6 + x7 + x8 + x9
}

#[ffi_function]
pub fn ptr(x: *const i64) -> *const i64 {
    x
}

#[ffi_function]
pub fn ptr_ptr(x: *const *const i64) -> *const *const i64 {
    x
}

/// # Safety
///
/// Parameter x must point to valid data.
#[ffi_function]
#[allow(unused_unsafe)]
pub unsafe fn ptr_mut(x: *mut i64) -> *mut i64 {
    unsafe { *x = -*x };
    x
}

#[ffi_function]
pub fn ref_simple(x: &i64) -> &i64 {
    x
}

#[ffi_function]
pub fn ref_mut_simple(x: &mut i64) -> &mut i64 {
    *x = -*x;
    x
}

#[ffi_function]
pub fn ref_option(x: Option<&i64>) -> bool {
    x.is_some()
}

#[ffi_function]
pub fn ref_mut_option(x: Option<&mut i64>) -> bool {
    x.is_some()
}

#[ffi_function]
pub fn call_tupled(x: Tupled) -> Tupled {
    Tupled(x.0 * 2)
}

#[ffi_function]
pub fn repr_transparent(x: Transparent, _r: &Transparent) -> Transparent {
    x
}

#[ffi_function]
pub fn complex_args_1(_a: Vec3f32, _b: Option<&Tupled>) -> FFIError {
    FFIError::Ok
}

#[ffi_function]
pub fn callback(callback: Callbacku8u8, value: u8) -> u8 {
    callback(value)
}

#[ffi_function]
pub fn callback_marshalled(callback: CallbackCharArray, value: CharArray) {
    callback(value)
}

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
pub fn array_1(x: Array) -> u8 {
    x.data[0]
}

#[ffi_function]
pub fn array_2() -> Array {
    Array {
        data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
    }
}

#[ffi_function]
pub fn array_3(arr: &mut Array) {
    arr.data[0] = 42;
}

#[ffi_function]
pub fn nested_array_1() -> NestedArray {
    NestedArray {
        field_enum: EnumRenamedXYZ::X,
        field_vec: Vec3f32 { x: 1.0, y: 2.0, z: 3.0 },
        field_bool: true,
        field_int: 42,
        field_array: [1, 2, 3, 4, 5],
        field_array_2: [6, 7, 8, 9, 10],
        field_struct: Array {
            data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        },
    }
}

#[ffi_function]
pub fn nested_array_2(result: &mut NestedArray) {
    result.field_enum = EnumRenamedXYZ::X;
    result.field_vec = Vec3f32 { x: 1.0, y: 2.0, z: 3.0 };
    result.field_bool = true;
    result.field_int = 42;
    result.field_array = [1, 2, 3, 4, 5];
    result.field_struct = Array {
        data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
    };
}

#[ffi_function]
pub fn nested_array_3(input: NestedArray) -> u8 {
    input.field_struct.data[1]
}

#[ffi_function]
pub fn char_array_1() -> CharArray {
    let mut result = CharArray {
        str: FixedString { data: [0; 32] },
        str_2: FixedString { data: [0; 32] },
    };

    result.str.data[..14].copy_from_slice(b"Hello, World!\0");

    result
}

#[ffi_function]
pub fn char_array_2(arr: CharArray) -> CharArray {
    arr
}

#[ffi_function]
pub fn char_array_3(arr: &CharArray) -> u8 {
    arr.str.data[0]
}

#[ffi_function]
pub fn bool_field(x: BoolField) -> bool {
    x.val
}

#[ffi_function]
pub fn renamed(x: StructRenamedXYZ) -> EnumRenamedXYZ {
    x.e
}

// Apparently this is not valid C?
// https://stackoverflow.com/questions/11656532/returning-an-array-using-c
// #[ffi_function]
//
// pub fn array_2(x: [u8; 16]) -> [u8; 16] {
//     x
// }

/// This function has documentation.
#[ffi_function]
pub fn documented(_x: StructDocumented) -> EnumDocumented {
    EnumDocumented::A
}

#[ffi_function]
pub fn ambiguous_1(x: ambiguous1::Vec) -> ambiguous1::Vec {
    x
}

#[ffi_function]
pub fn ambiguous_2(x: ambiguous2::Vec) -> ambiguous2::Vec {
    x
}

#[ffi_function]
pub fn ambiguous_3(x: ambiguous1::Vec, y: ambiguous2::Vec) -> bool {
    (x.x as f64 - y.x).abs() < 0.5
}

#[ffi_function]
pub fn namespaced_type(x: common::Vec) -> common::Vec {
    x
}

#[ffi_function]
pub fn namespaced_inner_option(x: FFIOption<common::Vec>) -> FFIOption<common::Vec> {
    x
}

#[ffi_function]
pub fn namespaced_inner_slice(x: FFISlice<common::Vec>) -> FFISlice<common::Vec> {
    x
}

#[ffi_function]
pub fn namespaced_inner_slice_mut(x: FFISliceMut<common::Vec>) -> FFISliceMut<common::Vec> {
    x
}

#[ffi_function]
#[allow(unreachable_code)]
pub fn panics() -> FFIError {
    panics_and_errors_to_ffi_enum(
        || {
            panic!("Oh no");
            Ok::<(), Error>(())
        },
        here!(),
    )
}

#[ffi_function]
pub fn sleep(millis: u64) {
    std::thread::sleep(Duration::from_millis(millis));
}

#[ffi_function]
pub fn weird_1(_x: Weird1<u32>, _y: Weird2<u8, 5>) -> bool {
    true
}

#[ffi_function]
pub fn visibility(_x: Visibility1, _y: Visibility2) {}
