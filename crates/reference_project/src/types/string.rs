use interoptopus::ffi;

#[ffi]
pub struct UseCStrPtr<'a> {
    pub ascii_string: ffi::CStrPtr<'a>,
}

#[ffi]
#[derive(Clone)]
pub struct UseString {
    pub s1: ffi::String,
    pub s2: ffi::String,
}
