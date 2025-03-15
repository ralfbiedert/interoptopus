use interoptopus::ffi_type;
use interoptopus::lang::rust::CTypeInfo;
use std::fmt::Debug;

#[ffi_type]
pub struct ExtraType<T> {
    pub x: T,
}

#[ffi_type]
pub struct Generic<'a, T>
where
    T: 'static,
    T: CTypeInfo,
{
    pub x: &'a T,
}

#[ffi_type(opaque)]
pub struct Generic2<T>
where
    T: CTypeInfo,
{
    pub x: T,
}

#[ffi_type(opaque, name = "Generic3")]
pub struct Generic3<T> {
    pub x: T,
}

pub trait Helper {}

impl Helper for u8 {}

#[ffi_type(opaque, name = "Generic4")]
pub struct Generic4<T>
where
    T: Helper,
{
    pub x: T,
}

#[ffi_type]
pub struct Weird1<T: Clone>
where
    T: Copy + Copy,
{
    x: T,
}

#[ffi_type]
pub struct Weird2<'a, T: Clone, const N: usize>
where
    T: Copy + Copy + 'a,
    T: Debug,
{
    t: T,
    a: [T; N],
    r: &'a u8,
}

pub mod associated_types {
    use interoptopus::ffi_type;

    pub trait Helper {
        type X;
    }

    #[ffi_type]
    pub struct Chicken(u8);

    #[ffi_type]
    pub struct Cow(u16);

    impl Helper for Chicken {
        type X = Cow;
    }

    #[ffi_type]
    pub struct FieldsViaAssociatedType {
        pub x: <Chicken as Helper>::X,
    }
}
