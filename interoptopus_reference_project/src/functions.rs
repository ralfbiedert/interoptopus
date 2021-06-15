use crate::types::{some_foreign_type, Callback, Empty, FFIError, Opaque, SomeForeignType, Vec3f32, Generic, Phantom, EnumDocumented, StructDocumented};
use interoptopus::ffi_function;
use std::ptr::null;

#[ffi_function]
#[no_mangle]
pub extern "C" fn primitive_void() {}

#[ffi_function]
#[no_mangle]
pub extern "C" fn primitive_void2() -> () {}

#[ffi_function]
#[no_mangle]
pub extern "C" fn primitive_bool(x: bool) -> bool {
    x
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn primitive_u8(x: u8) -> u8 {
    x
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn primitive_u16(x: u16) -> u16 {
    x
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn primitive_u32(x: u32) -> u32 {
    x
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn primitive_u64(x: u64) -> u64 {
    x
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn primitive_i8(x: i8) -> i8 {
    x
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn primitive_i16(x: i16) -> i16 {
    x
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn primitive_i32(x: i32) -> i32 {
    x
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn primitive_i64(x: i64) -> i64 {
    x
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

#[ffi_function]
#[no_mangle]
pub extern "C" fn ptr_mut(x: *mut i64) -> *mut i64 {
    x
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ptr_simple(x: &i64) -> &i64 {
    x
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ptr_simple_mut(x: &mut i64) -> &mut i64 {
    x
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ptr_option(x: Option<&i64>) -> &i64 {
    x.unwrap()
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ptr_option_mut(x: Option<&mut i64>) -> &mut i64 {
    x.unwrap()
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn complex_1(_a: Vec3f32, _b: Option<&Empty>) -> FFIError {
    FFIError::Ok
}

#[ffi_function(surrogates(_cmplx = "some_foreign_type"))]
#[no_mangle]
pub extern "C" fn complex_2(_cmplx: SomeForeignType) -> *const Opaque {
    null()
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn callback(callback: Callback, value: u8) -> u8 {
    callback(value)
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn generic(_x: Generic<u32>, _y: Phantom<u8>) -> u8 { 0 }


/// This function has documentation.
#[ffi_function]
#[no_mangle]
pub extern "C" fn documented(_x: StructDocumented) -> EnumDocumented {
    EnumDocumented::A
}

