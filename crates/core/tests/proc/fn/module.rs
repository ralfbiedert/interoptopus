use interoptopus::ffi_function;

#[ffi_function(module = "foo")]
fn foo1() {}

#[ffi_function(module = common)]
fn foo2() {}

fn main() {}
