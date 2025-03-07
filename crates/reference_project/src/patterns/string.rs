use crate::patterns::result::FFIError;
use crate::types::{UseCStrPtr, UseUtf8String};
use interoptopus::ffi_function;
use interoptopus::patterns::result::FFIResult;
use interoptopus::patterns::slice::FFISlice;
use interoptopus::patterns::string::{CStrPointer, Utf8String};

#[ffi_function]
pub fn pattern_ascii_pointer_1(x: CStrPointer) -> u32 {
    x.as_c_str().map(|x| x.to_bytes().len()).unwrap_or(0) as u32
}

#[ffi_function]
pub fn pattern_ascii_pointer_2() -> CStrPointer<'static> {
    CStrPointer::empty()
}

// NOTE: In some languages (C#) this can be a bad idea, because
// your input parameter will be automatically marshalled, but once
// the call returns that marshalling will stop, and by the time
// you use the output parameter again that helper struct got
// deallocated.
#[ffi_function]
pub fn pattern_ascii_pointer_3(x: CStrPointer) -> CStrPointer {
    x
}

#[ffi_function]
pub fn pattern_ascii_pointer_4(x: CStrPointer, l: u32) -> CStrPointer {
    let bytes = x.as_c_str().unwrap().to_bytes();
    CStrPointer::from_slice_with_nul(&bytes[l as usize..]).unwrap()
}

#[ffi_function]
pub fn pattern_ascii_pointer_5(x: CStrPointer, i: u32) -> u8 {
    let bytes = x.as_c_str().unwrap().to_bytes();
    bytes[i as usize]
}

//  Disabled for now
// #[ffi_function]
// pub fn pattern_ascii_pointer_len(x: CStrPointer, y: UseAsciiStringPattern) -> u32 {
//     let x1 = x.as_str().map(|x| x.len()).unwrap_or(0);
//     let x2 = y.ascii_string.as_str().map(|x| x.len()).unwrap_or(0);
//     (x1 + x2) as u32
// }

#[ffi_function]
pub fn pattern_ascii_pointer_return_slice() -> FFISlice<'static, UseCStrPtr<'static>> {
    FFISlice::empty()
}

#[ffi_function]
pub fn pattern_string_1(x: Utf8String) -> Utf8String {
    x
}

#[ffi_function]
pub fn pattern_string_2(x: Utf8String) -> u32 {
    x.into_string().len() as u32
}

#[ffi_function]
pub fn pattern_string_3() -> Utf8String {
    Utf8String::from_string("pattern_string_3".to_string())
}

#[ffi_function]
pub fn pattern_string_4(x: UseUtf8String) -> UseUtf8String {
    x
}

#[ffi_function]
pub fn pattern_string_5(x: UseUtf8String) -> FFIResult<UseUtf8String, FFIError> {
    FFIResult::ok(x)
}

#[ffi_function]
pub fn pattern_string_6a(x: &UseUtf8String) -> FFIResult<(), FFIError> {
    FFIResult::ok(())
}

#[ffi_function]
pub fn pattern_string_6b(y: &mut UseUtf8String) -> FFIResult<(), FFIError> {
    *y = UseUtf8String { s1: Utf8String::from_string("s1".to_string()), s2: Utf8String::from_string("s2".to_string()) };
    FFIResult::ok(())
}
