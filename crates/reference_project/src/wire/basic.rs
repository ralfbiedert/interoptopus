use interoptopus::ffi;
use interoptopus::wire::Wire;
use std::collections::HashMap;

#[ffi]
pub fn wire_accept_string_1(_input: Wire<String>) {}

#[ffi]
pub struct MyString {
    pub x: String,
}

#[ffi]
pub fn wire_accept_string_2(_input: Wire<MyString>) {}

#[ffi]
pub enum MyEnum {
    A,
    B,
    C,
}

#[ffi]
pub fn wire_accept_enum_1(mut input: Wire<MyEnum>) -> u32 {
    let val = input.unwire();
    val as u32
}

#[ffi]
pub enum DataEnum {
    S(String),
    V(Vec<u8>),
    H(HashMap<String, MyEnum>),
}

#[ffi]
pub fn wire_accept_enum_2(mut x: Wire<DataEnum>) -> u32 {
    match x.unwire() {
        DataEnum::S(x) => x.len() as u32,
        DataEnum::V(x) => x.len() as u32,
        DataEnum::H(x) => x.len() as u32,
    }
}
