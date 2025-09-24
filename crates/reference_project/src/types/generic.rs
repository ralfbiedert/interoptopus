use interoptopus::ffi;
use interoptopus::lang::types::TypeInfo;
use std::fmt::Debug;

#[ffi]
pub struct ExtraType<T: TypeInfo> {
    pub x: T,
}

#[ffi]
pub struct Generic<'a, T>
where
    T: 'static,
    T: TypeInfo,
{
    pub x: &'a T,
}

#[ffi(opaque)]
pub struct Generic2<T>
where
    T: TypeInfo,
{
    pub x: T,
}

#[ffi(opaque, name = "Generic3")]
pub struct Generic3<T> {
    pub x: T,
}

pub trait Helper {}

impl Helper for u8 {}

#[ffi(opaque, name = "Generic4")]
pub struct Generic4<T>
where
    T: Helper + TypeInfo,
{
    pub x: T,
}

#[ffi]
pub struct Weird1<T: Clone>
where
    T: Copy + Copy + TypeInfo,
{
    x: T,
}

#[ffi]
pub struct Weird2<'a, T: Clone, const N: usize>
where
    T: Copy + Copy + 'a,
    T: Debug + TypeInfo,
{
    t: T,
    a: [T; N],
    r: &'a u8,
}

pub mod associated_types {
    use interoptopus::ffi;

    pub trait Helper {
        type X;
    }

    #[ffi]
    pub struct Chicken(u8);

    #[ffi]
    pub struct Cow(u16);

    impl Helper for Chicken {
        type X = Cow;
    }

    #[ffi]
    pub struct FieldsViaAssociatedType {
        pub x: <Chicken as Helper>::X,
    }
}
