use interoptopus::ffi;
use interoptopus::pattern::primitive::{Bool, CChar};

#[ffi]
pub fn pattern_ffi_bool(ffi_bool: Bool) -> Bool {
    !ffi_bool
}

#[ffi]
pub fn pattern_ffi_cchar(ffi_cchar: CChar) -> CChar {
    ffi_cchar
}

#[ffi]
pub fn pattern_ffi_cchar_const_pointer(ffi_cchar: *const CChar) -> *const CChar {
    ffi_cchar
}

#[ffi]
pub fn pattern_ffi_cchar_mut_pointer(ffi_cchar: *mut CChar) -> *mut CChar {
    ffi_cchar
}
