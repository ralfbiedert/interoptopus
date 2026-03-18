use interoptopus::lang::types::SerializationError;
use interoptopus::wire::Wire;
use std::collections::HashMap;

#[test]
fn string_roundtrip() -> Result<(), SerializationError> {
    let s = "hello interoptopus".to_string();
    let mut wire = Wire::try_from(s.clone())?;
    assert_eq!(wire.try_unwire()?, s);
    Ok(())
}

#[test]
fn string_empty() -> Result<(), SerializationError> {
    let s = String::new();
    let mut wire = Wire::try_from(s.clone())?;
    assert_eq!(wire.try_unwire()?, s);
    Ok(())
}

#[test]
fn string_unicode() -> Result<(), SerializationError> {
    let s = "selâm aleyküm 🌍".to_string();
    let mut wire = Wire::try_from(s.clone())?;
    assert_eq!(wire.try_unwire()?, s);
    Ok(())
}

#[test]
fn vec_of_u32() -> Result<(), SerializationError> {
    let v = vec![1_u32, 2, 3, 4, 5];
    let mut wire = Wire::try_from(v.clone())?;
    assert_eq!(wire.try_unwire()?, v);
    Ok(())
}

#[test]
fn vec_empty() -> Result<(), SerializationError> {
    let v = Vec::<u64>::new();
    let mut wire = Wire::try_from(v.clone())?;
    assert_eq!(wire.try_unwire()?, v);
    Ok(())
}

#[test]
fn vec_of_strings() -> Result<(), SerializationError> {
    let v = vec!["one".to_string(), "two".to_string(), "three".to_string()];
    let mut wire = Wire::try_from(v.clone())?;
    assert_eq!(wire.try_unwire()?, v);
    Ok(())
}

#[test]
fn vec_of_vecs() -> Result<(), SerializationError> {
    let v = vec![vec![1_u8, 2, 3], vec![], vec![4, 5]];
    let mut wire = Wire::try_from(v.clone())?;
    assert_eq!(wire.try_unwire()?, v);
    Ok(())
}

#[test]
fn hashmap_string_to_u32() -> Result<(), SerializationError> {
    let mut m = HashMap::<String, u32>::new();
    m.insert("alpha".into(), 1);
    m.insert("beta".into(), 2);
    m.insert("gamma".into(), 3);
    let mut wire = Wire::try_from(m.clone())?;
    assert_eq!(wire.try_unwire()?, m);
    Ok(())
}

#[test]
fn hashmap_u32_to_string() -> Result<(), SerializationError> {
    let mut m = HashMap::<u32, String>::new();
    m.insert(10, "ten".into());
    m.insert(20, "twenty".into());
    let mut wire = Wire::try_from(m.clone())?;
    assert_eq!(wire.try_unwire()?, m);
    Ok(())
}

#[test]
fn hashmap_empty() -> Result<(), SerializationError> {
    let m = HashMap::<String, Vec<u8>>::new();
    let mut wire = Wire::try_from(m.clone())?;
    assert_eq!(wire.try_unwire()?, m);
    Ok(())
}

#[test]
fn hashmap_nested_vec_values() -> Result<(), SerializationError> {
    let mut m = HashMap::<String, Vec<u32>>::new();
    m.insert("primes".into(), vec![2, 3, 5, 7, 11]);
    m.insert("empty".into(), vec![]);
    m.insert("single".into(), vec![42]);
    let mut wire = Wire::try_from(m.clone())?;
    assert_eq!(wire.try_unwire()?, m);
    Ok(())
}

#[test]
fn hashmap_of_hashmaps() -> Result<(), SerializationError> {
    let mut inner1 = HashMap::<String, u32>::new();
    inner1.insert("x".into(), 1);
    inner1.insert("y".into(), 2);

    let mut inner2 = HashMap::<String, u32>::new();
    inner2.insert("a".into(), 10);

    let mut outer = HashMap::<String, HashMap<String, u32>>::new();
    outer.insert("first".into(), inner1);
    outer.insert("second".into(), inner2);

    let mut wire = Wire::try_from(outer.clone())?;
    assert_eq!(wire.try_unwire()?, outer);
    Ok(())
}
