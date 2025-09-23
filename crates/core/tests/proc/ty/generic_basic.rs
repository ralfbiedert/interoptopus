use interoptopus::{ffi_type, lang::types::TypeInfo};

#[ffi_type]
pub struct Foo1<T: TypeInfo> {
    x: T,
}

#[ffi_type]
pub struct Foo2<T>
where
    T: TypeInfo,
{
    x: T,
}

fn main() {}
