use interoptopus::pattern::option::Option;
use interoptopus::{ffi, ffi_function, ffi_type};

#[ffi_type]
pub struct Inner {
    pub x: f32,
}

#[ffi_function]
pub fn pattern_ffi_option_1(x: Option<Inner>) -> Option<Inner> {
    x
}

#[ffi_function]
pub fn pattern_ffi_option_2(x: Option<Inner>) -> Inner {
    x.into_option().unwrap_or(Inner { x: f32::NAN })
}

#[ffi_function]
pub fn pattern_ffi_option_3(
    x: ffi::Option<ffi::Option<ffi::Result<ffi::Option<ffi::String>, super::result::Error>>>,
) -> ffi::Option<ffi::Option<ffi::Result<ffi::Option<ffi::String>, super::result::Error>>> {
    x
}
