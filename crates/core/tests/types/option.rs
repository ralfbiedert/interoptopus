use interoptopus::ffi;

#[test]
fn option_variants() {
    assert!(ffi::Option::Some(0u32).is_some());
    assert!(!ffi::Option::Some(0u32).is_none());
    assert!(ffi::Option::<u32>::None.is_none());
    assert!(!ffi::Option::<u32>::None.is_some());
}

#[test]
fn option_default_is_none() {
    assert!(ffi::Option::<u32>::default().is_none());
}

#[test]
fn option_roundtrip() {
    let rt: Option<u32> = ffi::Option::from(Some(7)).into();
    assert_eq!(rt, Some(7));
    let rt: Option<u32> = ffi::Option::from(None).into();
    assert_eq!(rt, None);
}
