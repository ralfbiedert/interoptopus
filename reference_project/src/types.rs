//! All supported type patterns.

use interoptopus::lang::c::{CType, CompositeType, Field, PrimitiveType};
use interoptopus::lang::rust::CTypeInfo;
use interoptopus::patterns::slice::FFISlice;
use interoptopus::patterns::string::AsciiPointer;
use interoptopus::{callback, ffi_surrogates, ffi_type};
use std::fmt::Debug;
use std::marker::PhantomData;

pub trait Helper {}

impl Helper for u8 {}

// Let's assume we can't implement `CTypeInfo` for this.
#[repr(C)]
pub struct SomeForeignType {
    x: u32,
}

// Surrogate we can use instead of `SomeForeignType`
pub fn some_foreign_type() -> CType {
    let composite = CompositeType::new("SomeForeignType".to_string(), vec![Field::new("x".to_string(), CType::Primitive(PrimitiveType::U32))]);
    CType::Composite(composite)
}

/// Empty structs are only allowed as opaques.
#[ffi_type(opaque)]
#[repr(C)]
pub struct Empty {}

#[ffi_type(opaque)]
pub struct Opaque {
    _internal: *const Vec3f32,
}

#[ffi_type]
#[repr(C)]
pub struct Tupled(pub u8);

#[ffi_type]
#[repr(transparent)]
pub struct Transparent(Tupled);

#[ffi_type]
#[repr(C)]
pub struct Generic<'a, T>
where
    T: 'static,
    T: CTypeInfo,
{
    pub x: &'a T,
}

#[ffi_type(opaque)]
#[repr(C)]
pub struct Generic2<T>
where
    T: CTypeInfo,
{
    pub x: T,
}

#[ffi_type(opaque, name = "Generic3")]
#[repr(C)]
pub struct Generic3<T> {
    pub x: T,
}

#[ffi_type(opaque, name = "Generic4")]
#[repr(C)]
pub struct Generic4<T>
where
    T: Helper,
{
    pub x: T,
}

#[ffi_type(name = "StructRenamed")]
#[repr(C)]
pub struct StructRenamedXYZ {
    pub e: EnumRenamedXYZ,
}

#[ffi_type(skip(p))]
#[repr(C)]
pub struct Phantom<'a, T>
where
    T: 'static,
    T: CTypeInfo,
{
    pub x: u32,
    pub p: PhantomData<&'a T>,
}

#[ffi_type]
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Vec3f32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[ffi_type]
#[ffi_surrogates(foreign1 = "some_foreign_type", foreign2 = "some_foreign_type")]
#[repr(C)]
pub struct Container {
    pub foreign1: SomeForeignType,
    pub foreign2: SomeForeignType,
}

#[ffi_type]
#[repr(C)]
pub struct Array {
    pub data: [u8; 16],
}

#[ffi_type]
#[repr(C)]
pub struct GenericArray<T>
where
    T: CTypeInfo,
{
    pub data: [T; 16],
}

// TODO
// #[ffi_type]
// #[repr(C)]
// pub struct ConstGenericArray<T, const N: usize> {
//     data: [T; N]
// }

/// Documented enum.
#[ffi_type]
#[repr(C)]
pub enum EnumDocumented {
    /// Variant A.
    A,
    /// Variant B.
    B,
    /// Variant B.
    C,
}

#[ffi_type(name = "EnumRenamed")]
#[repr(C)]
pub enum EnumRenamedXYZ {
    X,
}

/// Documented struct.
#[ffi_type]
#[repr(C)]
pub struct StructDocumented {
    /// Documented field.
    pub x: f32,
}

#[ffi_type]
#[repr(C)]
pub struct ExtraType<T> {
    pub x: T,
}

#[ffi_type]
#[repr(C)]
pub struct UseAsciiStringPattern<'a> {
    pub ascii_string: AsciiPointer<'a>,
}

/// This can also be used for the `class` pattern.
#[ffi_type(opaque)]
#[repr(C)]
pub struct SomeContext {
    pub(crate) some_field: u32,
}

#[ffi_type]
#[repr(C)]
pub struct Weird1<T: Clone>
where
    T: Copy + Copy,
{
    x: T,
}

#[ffi_type]
#[repr(C)]
pub struct Weird2<'a, T: Clone, const N: usize>
where
    T: Copy + Copy + 'a,
    T: Debug,
{
    t: T,
    a: [T; N],
    r: &'a u8,
}

#[ffi_type(visibility(pblc = "public", prvt = "private"))]
#[repr(C)]
pub struct Visibility1 {
    // Be conservative with naming since some languages don't like `public` as a field.
    pblc: u8,
    pub prvt: u8,
}

#[ffi_type(visibility(_ = "public"))]
#[repr(C)]
pub struct Visibility2 {
    pblc1: u8,
    pblc2: u8,
}

#[ffi_type]
#[repr(C)]
#[repr(packed)]
pub struct Packed1 {
    pub x: u8,
    pub y: u16,
}

#[ffi_type]
#[repr(C, packed)]
pub struct Packed2 {
    pub x: u8,
    pub y: u16,
}

#[ffi_type]
#[repr(C)]
#[repr(align(2))]
pub struct Align1 {
    pub x: u8,
    pub y: u16,
}

#[ffi_type]
#[repr(C, align(64))]
pub struct Align2 {
    pub x: u8,
    pub y: u16,
}

// Doesn't need annotations.
pub type Callbacku8u8 = extern "C" fn(u8) -> u8;

// This does not work since we can't express the for<'x> bounds in our CTypeInfo implementation.
// pub type CallbackFFISlice = extern "C" fn(FFISlice<u8>) -> u8;

callback!(CallbackFFISlice(slice: FFISlice<u8>) -> u8);

pub mod ambiguous1 {
    use interoptopus::ffi_type;

    #[ffi_type(name = "Vec1")]
    #[repr(C)]
    pub struct Vec {
        pub x: f32,
        pub y: f32,
    }

    #[ffi_type(name = "Status1")]
    #[repr(C)]
    pub enum Status {
        X = 1,
        Y = 2,
    }
}

pub mod ambiguous2 {
    use interoptopus::ffi_type;

    #[ffi_type(name = "Vec2")]
    #[repr(C)]
    pub struct Vec {
        pub x: f64,
        pub z: f64,
    }

    #[ffi_type(name = "Status2")]
    #[repr(C)]
    pub enum Status {
        X = 100,
        Z = 200,
    }
}

pub mod common {
    use interoptopus::ffi_type;

    #[ffi_type(namespace = "common")]
    #[repr(C)]
    pub struct Vec {
        pub x: f64,
        pub z: f64,
    }
}

pub mod associated_types {
    use interoptopus::ffi_type;

    pub trait Helper {
        type X;
    }

    #[ffi_type]
    #[repr(C)]
    pub struct Chicken(u8);

    #[ffi_type]
    #[repr(C)]
    pub struct Cow(u16);

    impl Helper for Chicken {
        type X = Cow;
    }

    #[ffi_type]
    #[repr(C)]
    pub struct FieldsViaAssociatedType {
        pub x: <Chicken as Helper>::X,
    }
}
