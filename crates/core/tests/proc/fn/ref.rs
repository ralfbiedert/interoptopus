use interoptopus::ffi_function;

#[ffi_function]
fn foo1(_: &u8) {}

#[ffi_function]
fn foo2(_: &mut u8) {}

#[allow(dead_code)]
fn main() {}
