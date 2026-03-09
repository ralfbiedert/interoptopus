use interoptopus::ffi;
use std::os::raw::c_char;

#[test]
fn bool_works() {
    assert!(ffi::Bool::TRUE.is());
    assert!(!ffi::Bool::FALSE.is());
}

#[test]
fn cchar_works() {
    assert!(c_char::from(ffi::CChar::MAX) == c_char::MAX);
    assert!(ffi::CChar::from(c_char::MAX) == ffi::CChar::MAX);
}
