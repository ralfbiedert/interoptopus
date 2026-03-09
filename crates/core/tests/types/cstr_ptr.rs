use interoptopus::ffi;
use std::ffi::CString;

#[test]
fn can_create() {
    let s = "hello world";
    let cstr = CString::new(s).unwrap();

    let ptr_some = ffi::CStrPtr::from_cstr(&cstr);

    assert_eq!(s, ptr_some.as_str().unwrap());
}

#[test]
fn from_slice_with_nul_works() {
    let s = b"hello\0world";
    let ptr_some = ffi::CStrPtr::from_slice_with_nul(&s[..]).unwrap();

    assert_eq!("hello", ptr_some.as_str().unwrap());
}

#[test]
fn from_slice_with_nul_fails_if_not_nul() {
    let s = b"hello world";
    let ptr_some = ffi::CStrPtr::from_slice_with_nul(&s[..]);

    assert!(ptr_some.is_err());
}
