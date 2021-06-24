use interoptopus::patterns::option::FFIOption;
use interoptopus::{ffi_function, ffi_type};

#[ffi_type]
#[repr(C)]
pub struct Inner {
    x: f32,
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_ffi_option(ffi_slice: FFIOption<Inner>) -> FFIOption<Inner> {
    ffi_slice
}
