//! Functions using all supported type patterns.

use crate::patterns::success_enum::FFIError;
use crate::types::{
    ambiguous1, ambiguous2, common, some_foreign_type, Array, Callbacku8u8, Empty, EnumDocumented, Generic, Generic2, Generic3, Opaque, Phantom, SomeForeignType,
    StructDocumented, Tupled, Vec3f32, Weird1, Weird2,
};
use interoptopus::ffi_function;
use interoptopus::patterns::success_enum::panics_and_errors_to_ffi_enum;
use std::ptr::null;
use std::time::Duration;

#[ffi_function]
#[no_mangle]
pub extern "C" fn primitive_void() {}

#[ffi_function]
#[no_mangle]
#[allow(clippy::unused_unit)]
pub extern "C" fn primitive_void2() -> () {}

#[ffi_function]
#[no_mangle]
pub extern "C" fn primitive_bool(x: bool) -> bool {
    !x
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn primitive_u8(x: u8) -> u8 {
    u8::MAX - x
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn primitive_u16(x: u16) -> u16 {
    u16::MAX - x
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn primitive_u32(x: u32) -> u32 {
    u32::MAX - x
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn primitive_u64(x: u64) -> u64 {
    u64::MAX - x
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn primitive_i8(x: i8) -> i8 {
    -x
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn primitive_i16(x: i16) -> i16 {
    -x
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn primitive_i32(x: i32) -> i32 {
    -x
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn primitive_i64(x: i64) -> i64 {
    -x
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn many_args_5(x0: i64, x1: i64, x2: i64, x3: i64, x4: i64) -> i64 {
    x0 + x1 + x2 + x3 + x4
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn many_args_10(x0: i64, x1: i64, x2: i64, x3: i64, x4: i64, x5: i64, x6: i64, x7: i64, x8: i64, x9: i64) -> i64 {
    x0 + x1 + x2 + x3 + x4 + x5 + x6 + x7 + x8 + x9
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ptr(x: *const i64) -> *const i64 {
    x
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ptr_ptr(x: *const *const i64) -> *const *const i64 {
    x
}

/// # Safety
///
/// Parameter x must point to valid data.
#[ffi_function]
#[no_mangle]
#[allow(unused_unsafe)]
pub unsafe extern "C" fn ptr_mut(x: *mut i64) -> *mut i64 {
    unsafe { *x = -*x };
    x
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ref_simple(x: &i64) -> &i64 {
    x
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ref_mut_simple(x: &mut i64) -> &mut i64 {
    *x = -*x;
    x
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ref_option(x: Option<&i64>) -> bool {
    x.is_some()
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ref_mut_option(x: Option<&mut i64>) -> bool {
    x.is_some()
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn tupled(x: Tupled) -> Tupled {
    Tupled(x.0 * 2)
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn complex_args_1(_a: Vec3f32, _b: Option<&Empty>) -> FFIError {
    FFIError::Ok
}

#[ffi_function(surrogates(_cmplx = "some_foreign_type"))]
#[no_mangle]
pub extern "C" fn complex_args_2(_cmplx: SomeForeignType) -> *const Opaque {
    null()
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn callback(callback: Callbacku8u8, value: u8) -> u8 {
    callback(value)
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn generic_1(x: Generic<u32>, _y: Phantom<u8>) -> u32 {
    *x.x
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn generic_2(x: Generic<u8>, _y: Phantom<u8>) -> u8 {
    *x.x
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn generic_3(x: &Generic2<u8>) -> u8 {
    x.x
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn generic_4(x: &Generic3<u8>) -> u8 {
    x.x
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn array_1(x: Array) -> u8 {
    x.data[0]
}

// Apparently this is not valid C?
// https://stackoverflow.com/questions/11656532/returning-an-array-using-c
// #[ffi_function]
// #[no_mangle]
// pub extern "C" fn array_2(x: [u8; 16]) -> [u8; 16] {
//     x
// }

/// This function has documentation.
#[ffi_function]
#[no_mangle]
pub extern "C" fn documented(_x: StructDocumented) -> EnumDocumented {
    EnumDocumented::A
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ambiguous_1(x: ambiguous1::Vec) -> ambiguous1::Vec {
    x
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ambiguous_2(x: ambiguous2::Vec) -> ambiguous2::Vec {
    x
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ambiguous_3(x: ambiguous1::Vec, y: ambiguous2::Vec) -> bool {
    (x.x as f64 - y.x).abs() < 0.5
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn namespaced_type(x: common::Vec) -> common::Vec {
    x
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn panics() -> FFIError {
    panics_and_errors_to_ffi_enum(|| panic!("Oh no"))
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn sleep(millis: u64) {
    std::thread::sleep(Duration::from_millis(millis));
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn weird_1(_x: Weird1<u32>, _y: Weird2<u8, 5>) -> bool {
    true
}
