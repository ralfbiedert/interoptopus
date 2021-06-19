use crate::types::{
    ambiguous1, ambiguous2, some_foreign_type, CallbackFFISlice, Callbacku8u8, Context, Empty, EnumDocumented, FFIError, Generic, Opaque, Phantom, SomeForeignType,
    StructDocumented, UseAsciiStringPattern, Vec3f32,
};
use interoptopus::ffi_function;
use interoptopus::patterns::ascii_pointer::AsciiPointer;
use interoptopus::patterns::slice::FFISlice;
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
pub extern "C" fn ptr_option(x: Option<&i64>) -> bool {
    x.is_some()
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ptr_option_mut(x: Option<&mut i64>) -> bool {
    x.is_some()
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
pub extern "C" fn callback(callback: Callbacku8u8, value: u8) -> u8 {
    callback(value)
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn generic(x: Generic<u32>, _y: Phantom<u8>) -> u32 {
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
pub extern "C" fn pattern_ascii_pointer(x: AsciiPointer, y: UseAsciiStringPattern) -> u8 {
    let _ = dbg!(x.as_str());
    let _ = dbg!(y.ascii_string.as_str().unwrap());
    0
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_class_create(context_ptr: Option<&mut *mut Context>, value: u32) -> FFIError {
    let the_box = Box::new(Context { some_field: value });

    match context_ptr {
        None => FFIError::Null,
        Some(c) => {
            *c = Box::into_raw(the_box);
            FFIError::Ok
        }
    }
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_ffi_slice(ffi_slice: FFISlice<u32>) -> u32 {
    ffi_slice.as_slice().unwrap_or(&[]).len() as u32
}

#[ffi_function(debug)]
#[no_mangle]
pub extern "C" fn pattern_ffi_slice_delegate(callback: CallbackFFISlice) -> u8 {
    callback.call(FFISlice::from_slice(&[1, 2, 3]))
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_class_method(context: Option<&mut Context>) -> u32 {
    match context {
        None => 0,
        Some(c) => {
            dbg!(c.some_field);
            c.some_field = 2 * c.some_field;
            c.some_field
        }
    }
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_class_method_success_enum_ok(_context: Option<&mut Context>) -> FFIError {
    FFIError::Ok
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_class_method_success_enum_fail(_context: Option<&mut Context>) -> FFIError {
    FFIError::Fail
}

#[ffi_function]
#[no_mangle]
#[allow(unused_unsafe)]
pub unsafe extern "C" fn pattern_class_destroy(context_ptr: Option<&mut *mut Context>) -> FFIError {
    match context_ptr {
        None => FFIError::Null,
        Some(c) => {
            unsafe { Box::from_raw(*c) };
            FFIError::Ok
        }
    }
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
