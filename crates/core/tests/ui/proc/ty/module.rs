use interoptopus::ffi;

#[ffi(module = "foo")]
struct Foo1 {
    x: u8,
}

#[ffi(module = common)]
struct Foo2 {
    x: u8,
}

fn main() {}
