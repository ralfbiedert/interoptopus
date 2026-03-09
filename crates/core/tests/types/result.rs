use interoptopus::ffi;

#[test]
fn result_variants() {
    assert!(ffi::Result::<u32, u32>::Ok(1).is_ok());
    assert!(!ffi::Result::<u32, u32>::Err(1).is_ok());
    assert!(!ffi::Result::<u32, u32>::Panic.is_ok());
    assert!(!ffi::Result::<u32, u32>::Null.is_ok());
}

#[test]
fn result_roundtrip() {
    let ffi_ok: ffi::Result<u32, u32> = Result::Ok(42).into();
    assert_eq!(ffi_ok.unwrap(), 42);
    let ffi_err: ffi::Result<u32, u32> = Result::Err(99).into();
    assert_eq!(ffi_err.unwrap_err(), 99);
}
