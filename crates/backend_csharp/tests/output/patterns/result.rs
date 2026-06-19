use interoptopus::{ffi, function};

#[ffi]
pub enum Error {
    Fail,
}

#[ffi(export = unique)]
pub fn roundtrip_result(x: ffi::Result<u32, Error>) -> ffi::Result<u32, Error> {
    x
}

#[test]
fn basic() {
    test_output!("Interop.cs", [function!(roundtrip_result)]);
}
