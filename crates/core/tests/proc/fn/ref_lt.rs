use interoptopus::ffi;

#[ffi]
fn foo1<'a>(_: &'a u8) {}

#[ffi]
fn foo2<'a>(_: &'a mut u8) {}

fn main() {}
