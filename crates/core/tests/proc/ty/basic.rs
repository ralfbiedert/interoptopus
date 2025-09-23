use interoptopus::ffi_type;

#[ffi_type]
struct Foo {
    x: u8,
}

#[ffi_type]
enum Bar {
    A,
    B,
}

#[allow(dead_code)]
fn main() {}
