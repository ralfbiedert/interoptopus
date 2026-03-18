use interoptopus::{ffi, lang::types::TypeInfo};

#[ffi]
pub struct Foo1<T: TypeInfo> {
    x: T,
}

#[ffi]
pub struct Foo2<T>
where
    T: TypeInfo,
{
    x: T,
}

fn main() {}
