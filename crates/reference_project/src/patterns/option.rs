use interoptopus::ffi;
use interoptopus::ffi::{Option, Result, String};

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
pub fn pattern_ffi_option_3(x: Option<Option<Result<Option<String>, super::result::Error>>>) -> Option<Option<Result<Option<String>, super::result::Error>>> {
    x
}
