use interoptopus::ffi;

#[ffi]
fn foo1(_: &u8) {}

#[ffi]
fn foo2(_: &mut u8) {}

fn main() {}
