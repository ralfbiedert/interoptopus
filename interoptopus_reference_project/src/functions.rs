//! Functions using all supported type patterns.

use crate::patterns::service_generated::some_rust_module::Error;
use crate::patterns::success_enum::FFIError;
use crate::types::{
    ambiguous1, ambiguous2, common, some_foreign_type, Callbacku8u8, Empty, EnumDocumented, Generic, Opaque, Phantom, SomeForeignType, StructDocumented, Tupled, Vec3f32,
};
use interoptopus::ffi_function;
use interoptopus::patterns::success_enum::panics_and_errors_to_ffi_error;
use std::ptr::null;

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
    panics_and_errors_to_ffi_error(|| panic!("Oh no"))
}
