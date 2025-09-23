use interoptopus::ffi_type;

#[ffi_type]
pub struct Foo<T> {
    x: T,
}

#[allow(dead_code)]
fn main() {}
