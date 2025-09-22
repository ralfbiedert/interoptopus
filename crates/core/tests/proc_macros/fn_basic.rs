use crate::proc_macros::ty_basic::EnumPayload;
use interoptopus::ffi;
use interoptopus::lang::types::TypeInfo;
use interoptopus::pattern::result::{panic_to_result, result_to_ffi};
use interoptopus_proc::{ffi_function, ffi_type};

#[ffi_type]
pub struct Packed1(u8);

#[ffi_type]
pub struct Packed2(u8);

#[ffi_type]
#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum Error {
    Fail,
}

#[ffi_type]
pub struct Generic<'a, T>
where
    T: 'static,
    T: TypeInfo,
{
    pub x: &'a T,
}

#[ffi_function]
pub fn alignment_1(a: Packed1) -> Packed2 {
    Packed2(a.0)
}

#[ffi_function]
pub fn alignment_2(_x: ffi::String) {}

#[ffi_function]
pub fn behavior_panics_via_result() -> ffi::Result<(), Error> {
    panic_to_result(|| result_to_ffi(|| Ok(())))
}

/// Blah
///
/// Foo
#[ffi_function(export = "abc")]
pub fn generic_1c<'a>(_x: Option<&'a Generic<'a, u8>>, y: &Generic<'a, u8>) -> u8 {
    *y.x
}

/// # Safety
///
/// Parameter x must point to valid data.
#[ffi_function]
#[allow(unused_unsafe)]
pub unsafe fn ptr3(x: *mut i64) -> *mut i64 {
    unsafe { *x = -*x };
    x
}

#[ffi_function]
pub fn ref5(x: &mut EnumPayload) {
    *x = EnumPayload::C(123);
}

#[ffi_function]
pub fn test_underscore_params(_: u32, _: ffi::String, x: u8) -> u8 {
    x + 1
}

#[test]
fn test() {}
