use interoptopus::ffi_function;
use interoptopus::patterns::primitives::{FFIBool, FFICChar};

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_ffi_bool(ffi_bool: FFIBool) -> FFIBool {
    !ffi_bool
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_ffi_cchar(ffi_cchar: FFICChar) -> FFICChar {
    FFICChar::MIN
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_ffi_cchar_const_pointer(ffi_cchar: *const FFICChar) -> *const FFICChar {
    ffi_cchar
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_ffi_cchar_mut_pointer(ffi_cchar: *mut FFICChar) -> *mut FFICChar {
    ffi_cchar
}
