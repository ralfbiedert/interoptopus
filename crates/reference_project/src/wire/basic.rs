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
pub fn wire_accept_string_3(mut input: Wire<Vec<String>>) {
    _ = input.unwire();
}

#[ffi]
pub fn wire_accept_string_4(mut input: Wire<HashMap<String, String>>) {
    _ = input.unwire();
}
