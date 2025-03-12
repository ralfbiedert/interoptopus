use interoptopus::ffi_function;
use interoptopus::patterns::primitive::{Bool, CChar};

#[ffi_function]
pub fn pattern_ffi_bool(ffi_bool: Bool) -> Bool {
    !ffi_bool
}

#[ffi_function]
pub fn pattern_ffi_cchar(ffi_cchar: CChar) -> CChar {
    ffi_cchar
}

#[ffi_function]
pub fn pattern_ffi_cchar_const_pointer(ffi_cchar: *const CChar) -> *const CChar {
    ffi_cchar
}

#[ffi_function]
pub fn pattern_ffi_cchar_mut_pointer(ffi_cchar: *mut CChar) -> *mut CChar {
    ffi_cchar
}
