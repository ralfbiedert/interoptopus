use interoptopus::ffi;
use interoptopus::pattern::option::Option;

#[ffi]
pub struct Inner {
    pub x: f32,
}

#[ffi]
pub fn pattern_ffi_option_1(x: Option<Inner>) -> Option<Inner> {
    x
}

#[ffi]
pub fn pattern_ffi_option_2(x: Option<Inner>) -> Inner {
    x.into_option().unwrap_or(Inner { x: f32::NAN })
}

#[ffi]
pub fn pattern_ffi_option_3(
    x: ffi::Option<ffi::Option<ffi::Result<ffi::Option<ffi::String>, super::result::Error>>>,
) -> ffi::Option<ffi::Option<ffi::Result<ffi::Option<ffi::String>, super::result::Error>>> {
    x
}
