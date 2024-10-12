use interoptopus::patterns::option::FFIOption;
use interoptopus::{ffi_function, ffi_type};

#[ffi_type]
#[repr(C)]
pub struct Inner {
    x: f32,
}

#[ffi_function]
pub fn pattern_ffi_option_1(ffi_slice: FFIOption<Inner>) -> FFIOption<Inner> {
    ffi_slice
}

#[ffi_function]
pub fn pattern_ffi_option_2(ffi_slice: FFIOption<Inner>) -> Inner {
    ffi_slice.into_option().unwrap_or(Inner { x: f32::NAN })
}
