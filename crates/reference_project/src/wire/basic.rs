use interoptopus::ffi;
use interoptopus::wire::Wire;

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
