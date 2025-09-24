use interoptopus::ffi;

#[ffi]
struct Foo {
    x: u8,
}

#[ffi]
enum Bar {
    A,
    B,
}

fn main() {}
