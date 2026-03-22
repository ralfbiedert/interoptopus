use interoptopus::wire::Wire;

#[test]
fn clone_roundtrips() {
    let value = 1234u32;
    let original = Wire::<u32>::from(value);
    let mut clone = original.clone();
    assert_eq!(clone.unwire(), value);
}
