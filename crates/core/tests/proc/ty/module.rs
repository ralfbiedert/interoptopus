use interoptopus::ffi_type;

#[ffi_type(module = "foo")]
struct Foo1 {
    x: u8,
}

#[ffi_type(module = common)]
struct Foo2 {
    x: u8,
}

fn main() {}
