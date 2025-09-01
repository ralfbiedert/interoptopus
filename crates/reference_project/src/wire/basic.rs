use interoptopus::wire::Wire;
use interoptopus::{ffi_function, ffi_type};

// TODO: This fails to codegen on C#
#[ffi_function]
fn wire_accept_string_1(_input: Wire<String>) {}

#[ffi_type(wired)]
pub struct MyString {
    pub x: String,
}

#[ffi_function]
fn wire_accept_string_2(_input: Wire<MyString>) {}
