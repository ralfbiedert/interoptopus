use interoptopus::ffi_function;

#[ffi_function]
extern "C" fn foo() {}

#[allow(dead_code)]
fn main() {}
