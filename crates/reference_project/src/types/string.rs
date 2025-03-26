use interoptopus::{ffi, ffi_type};

#[ffi_type]
pub struct UseCStrPtr<'a> {
    pub ascii_string: ffi::CStrPtr<'a>,
}

#[ffi_type]
#[derive(Clone)]
pub struct UseString {
    pub s1: ffi::String,
    pub s2: ffi::String,
}
