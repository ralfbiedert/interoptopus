use crate::patterns::success_enum::FFIError;
use crate::types::Context;
use interoptopus::{ffi_function, pattern_service_manual};

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_service_create(context_ptr: Option<&mut *mut Context>, value: u32) -> FFIError {
    let the_box = Box::new(Context { some_field: value });

    match context_ptr {
        None => FFIError::Null,
        Some(c) => {
            *c = Box::into_raw(the_box);
            FFIError::Ok
        }
    }
}

/// # Safety
///
/// This function may only be called with a context returned by a succeeding `pattern_service_create`.
#[ffi_function]
#[no_mangle]
#[allow(unused_unsafe)]
pub unsafe extern "C" fn pattern_service_destroy(context_ptr: Option<&mut *mut Context>) -> FFIError {
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
pub extern "C" fn pattern_service_method(context: Option<&mut Context>) -> u32 {
    match context {
        None => 0,
        Some(c) => {
            dbg!(c.some_field);
            c.some_field *= 2;
            c.some_field
        }
    }
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_service_method_success_enum_ok(_context: Option<&mut Context>) -> FFIError {
    FFIError::Ok
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_service_method_success_enum_fail(_context: Option<&mut Context>) -> FFIError {
    FFIError::Fail
}

pattern_service_manual!(
    my_service_pattern_context,
    pattern_service_create,
    pattern_service_destroy,
    [
        pattern_service_method,
        pattern_service_method_success_enum_ok,
        pattern_service_method_success_enum_fail
    ]
);
