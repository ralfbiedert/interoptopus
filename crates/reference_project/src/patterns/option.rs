use interoptopus::pattern::option::Option;
use interoptopus::{ffi_function, ffi_type};

#[ffi_type]
pub struct Inner {
    x: f32,
}

#[ffi_function]
pub fn pattern_ffi_option_1(ffi_slice: Option<Inner>) -> Option<Inner> {
    ffi_slice
}

#[ffi_function]
pub fn pattern_ffi_option_2(ffi_slice: Option<Inner>) -> Inner {
    ffi_slice.into_option().unwrap_or(Inner { x: f32::NAN })
}
