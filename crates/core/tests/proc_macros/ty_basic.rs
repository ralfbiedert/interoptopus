#![allow(unused)]
use interoptopus::ffi;
use interoptopus::lang::types::TypeInfo;
use interoptopus_proc::ffi_type;
use std::fmt::Debug;
use std::marker::PhantomData;

// const _: () = {
//     assert!(X::WIRE_SAFE);
//     assert!(Y::WIRE_SAFE);
//     assert!(Z::WIRE_SAFE);
// };

// #[ffi_type]
struct ThisShouldFail;

#[ffi_type(opaque)]
struct ThisShouldWork;

#[ffi_type(packed)]
pub struct Packed1 {
    pub x: u8,
    pub y: u16,
}

#[ffi_type]
pub struct Array {
    pub data: [u8; 16],
}

#[ffi_type(name = "Vec2")]
pub struct Vec {
    pub x: f32,
    pub y: f32,
}

/// Documented struct.
/// abc.
/// def.
#[ffi_type]
#[derive(Clone)]
pub struct StructDocumented {
    /// Documented field.
    /// Other line
    pub x: f32,
}

#[ffi_type]
pub enum Layer3<T: TypeInfo> {
    A(Layer1<T>),
    B(Layer2<T>),
}

#[ffi_type(name = "EnumRenamed")]
pub enum EnumRenamedXYZ {
    X,
}

#[ffi_type]
pub enum EnumPayload {
    A,
    B(StructDocumented),
    C(u32),
    // We don't support these for now
    // D { x: Vec3f32 },
    // E(u8, u8, u8),
}

#[ffi_type]
pub enum EnumValue {
    A = 1,
    B = 123,
}

#[ffi_type]
pub struct Layer2<T: TypeInfo> {
    pub layer_1: Layer1<T>,
    pub vec: StructDocumented,
    pub the_enum: EnumPayload,
    // pub strings: ffi::Vec<ffi::String>,
}

#[ffi_type]
pub struct Layer1<T: TypeInfo> {
    pub maybe_1: ffi::Option<T>,
    // pub maybe_2: ffi::Vec<T>,
    pub maybe_3: T,
}

#[ffi_type]
pub struct UseCStrPtr<'a> {
    pub ascii_string: ffi::CStrPtr<'a>,
}

#[ffi_type]
pub struct Weird1<T: TypeInfo + Clone>
where
    T: Copy + Copy,
{
    x: T,
}

#[ffi_type]
pub struct Weird2<'a, T: TypeInfo + Clone, const N: usize>
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

#[ffi_type]
pub struct Phantom<'a, T>
where
    T: 'static,
    T: TypeInfo,
{
    pub x: u32,
    #[skip]
    pub p: PhantomData<&'a T>,
}

#[ffi_type(transparent, debug)]
pub struct Transparent<'a>(UseCStrPtr<'a>);

#[ffi_type(module = "abc")]
pub struct Vec2 {
    pub x: f64,
    pub z: f64,
}

#[ffi_type(opaque)]
pub struct Opaque {
    _internal: *const Packed1,
    _unused: (),
}

#[test]
fn test() {}
