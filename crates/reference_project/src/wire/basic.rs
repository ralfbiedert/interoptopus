use interoptopus::ffi_function;
use interoptopus::wire::Wire;

// TODO: This fails to codegen on C#
#[ffi]
fn wire_accept_string_1(_input: Wire<String>) {}

#[ffi]
pub struct MyString {
    pub x: String,
}

#[ffi]
pub fn wire_accept_string_2(_input: Wire<MyString>) {}
