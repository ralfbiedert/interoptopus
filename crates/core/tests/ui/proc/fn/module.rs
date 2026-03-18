use interoptopus::ffi;

#[ffi(module = "foo")]
fn foo1() {}

#[ffi(module = common)]
fn foo2() {}

fn main() {}
