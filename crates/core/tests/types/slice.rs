use interoptopus::ffi;

#[test]
fn can_create_ref() {
    let slice = &[0, 1, 2, 3, 5];
    let empty = ffi::Slice::<u8>::empty();
    let some = ffi::Slice::from(slice.as_slice());

    assert_eq!(empty.as_slice(), &[] as &[u8]);
    assert_eq!(some.as_slice(), slice);
}

#[test]
fn can_create_mut() {
    let slice = &mut [0, 1, 2, 3, 5];
    let empty = ffi::SliceMut::<u8>::empty();
    let mut some = ffi::SliceMut::from(slice.as_mut());
    let sub = &mut some[1..=2];

    sub[0] = 6;
    some[0] = 5;

    assert_eq!(empty.as_slice(), &[] as &[u8]);
    assert_eq!(slice, &[5, 6, 2, 3, 5]);
}

#[test]
fn multi_borrow_mut_slice() {
    let slice = &mut [0, 1, 2, 3, 5];
    let empty = ffi::SliceMut::<u8>::empty();
    let target: &mut [u8] = {
        let mut some = ffi::SliceMut::from(slice.as_mut());
        some.as_slice_mut()
    };
    let sub = &mut target[1..=2];

    sub[0] = 6;
    target[0] = 5;

    assert_eq!(empty.as_slice(), &[] as &[u8]);
    assert_eq!(slice, &[5, 6, 2, 3, 5]);
}
