use crate::patterns::result::Error;
use crate::types::string::{UseCStrPtr, UseString};
use interoptopus::ffi;
use interoptopus::ffi::{CStrPtr, Result, Slice};

#[ffi]
pub fn pattern_ascii_pointer_1(x: ffi::CStrPtr) -> u32 {
    x.as_c_str().map(|x| x.to_bytes().len()).unwrap_or(0) as u32
}

#[ffi]
pub fn pattern_ascii_pointer_2() -> ffi::CStrPtr<'static> {
    ffi::CStrPtr::empty()
}

// NOTE: In some languages (C#) this can be a bad idea, because
// your input parameter will be automatically marshalled, but once
// the call returns that marshalling will stop, and by the time
// you use the output parameter again that helper struct got
// deallocated.
#[ffi]
pub fn pattern_ascii_pointer_3(x: ffi::CStrPtr) -> ffi::CStrPtr {
    x
}

#[ffi]
pub fn pattern_ascii_pointer_4(x: ffi::CStrPtr, l: u32) -> ffi::CStrPtr {
    let bytes = x.as_c_str().unwrap().to_bytes();
    CStrPtr::from_slice_with_nul(&bytes[l as usize..]).unwrap()
}

#[ffi]
pub fn pattern_ascii_pointer_5(x: ffi::CStrPtr, i: u32) -> u8 {
    let bytes = x.as_c_str().unwrap().to_bytes();
    bytes[i as usize]
}

//  Disabled for now
// #[ffi]
// pub fn pattern_ascii_pointer_len(x: CStrPointer, y: UseAsciiStringPattern) -> u32 {
//     let x1 = x.as_str().map(|x| x.len()).unwrap_or(0);
//     let x2 = y.ascii_string.as_str().map(|x| x.len()).unwrap_or(0);
//     (x1 + x2) as u32
// }

#[ffi]
pub fn pattern_ascii_pointer_return_slice() -> ffi::Slice<'static, UseCStrPtr<'static>> {
    Slice::empty()
}

#[ffi]
pub fn pattern_string_1(x: ffi::String) -> ffi::String {
    x
}

#[ffi]
pub fn pattern_string_2(x: ffi::String) -> u32 {
    x.into_string().len() as u32
}

#[ffi]
pub fn pattern_string_3() -> ffi::String {
    ffi::String::from_string("pattern_string_3".to_string())
}

#[ffi]
pub fn pattern_string_4(x: UseString) -> UseString {
    x
}

#[ffi]
pub fn pattern_string_5(x: UseString) -> ffi::Result<UseString, Error> {
    Result::Ok(x)
}

#[ffi]
pub fn pattern_string_6a(_: &UseString) -> ffi::Result<(), Error> {
    ffi::Ok(())
}

#[ffi]
pub fn pattern_string_6b(y: &mut UseString) -> ffi::Result<(), Error> {
    *y = UseString { s1: ffi::String::from_string("s1".to_string()), s2: ffi::String::from_string("s2".to_string()) };
    Result::Ok(())
}

#[ffi]
pub fn pattern_string_7(x: ffi::Slice<ffi::String>, i: u64) -> ffi::Result<ffi::String, Error> {
    Result::Ok(x.as_slice()[i as usize].clone())
}

#[ffi]
pub fn pattern_string_8(x: ffi::Slice<UseString>, i: u64) -> ffi::Result<UseString, Error> {
    Result::Ok(x.as_slice()[i as usize].clone())
}

#[ffi]
pub fn pattern_string_9() -> ffi::Result<ffi::String, Error> {
    Result::Err(Error::Fail)
}

#[ffi]
pub fn pattern_string_10(_: ffi::String) {}

#[ffi]
pub fn pattern_string_11(_: &ffi::String) {}
