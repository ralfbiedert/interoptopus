use interoptopus::{ffi, ffi_type};

#[ffi_type]
#[derive(Clone)]
pub struct UseSliceAndVec<'a> {
    pub s1: ffi::Slice<'a, ffi::String>,
    pub s2: ffi::Vec<ffi::String>,
    pub s3: ffi::Vec<ffi::Slice<'a, u8>>,
    pub s4: ffi::Slice<'a, u8>,
}
