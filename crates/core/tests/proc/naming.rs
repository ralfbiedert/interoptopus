#![allow(dead_code)]

use interoptopus::lang::types::TypeInfo;
use interoptopus_proc::ffi;

#[allow(clippy::type_repetition_in_bounds)]
#[ffi]
struct Struct<T: TypeInfo> {
    t: T,
}

#[ffi]
enum Enum<T: TypeInfo> {
    A,
    B(T),
}

#[test]
fn generic_name_matches_prediction() {
    assert_eq!(Enum::<u32>::ty().name, "Enum<u32>");
    assert_eq!(Struct::<String>::ty().name, "Struct<String>");
}
