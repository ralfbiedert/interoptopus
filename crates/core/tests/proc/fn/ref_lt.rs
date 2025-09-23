use interoptopus::ffi_function;

#[ffi_function]
fn foo1<'a>(_: &'a u8) {}

#[ffi_function]
fn foo2<'a>(_: &'a mut u8) {}

#[allow(dead_code)]
fn main() {}
