use interoptopus::{ffi, function};

#[ffi]
pub enum Error {
    Fail,
}

#[ffi(export = unique)]
pub fn roundtrip_result(x: ffi::Result<u32, Error>) -> ffi::Result<u32, Error> {
    x
}

#[ffi(export = unique)]
pub fn roundtrip_result_unit_ok(x: ffi::Result<(), Error>) -> ffi::Result<(), Error> {
    x
}

#[ffi(export = unique)]
pub fn roundtrip_result_unit_err(x: ffi::Result<u32, ()>) -> ffi::Result<u32, ()> {
    x
}

#[test]
fn basic() {
    test_output!("Interop.cs", [function!(roundtrip_result), function!(roundtrip_result_unit_ok), function!(roundtrip_result_unit_err),]);
}

#[test]
fn common() {
    test_output!("Interop.Common.cs", [function!(roundtrip_result), function!(roundtrip_result_unit_ok), function!(roundtrip_result_unit_err),]);
}
