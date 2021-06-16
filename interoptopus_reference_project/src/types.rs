use interoptopus::ffi_type;
use interoptopus::lang::c::{CType, CompositeType, Field, PrimitiveType};
use interoptopus::lang::rust::CTypeInfo;
use interoptopus::patterns::ascii_pointer::AsciiPointer;
use std::marker::PhantomData;

// Let's assume we can't implement `CTypeInfo` for this.
#[repr(C)]
pub struct SomeForeignType {
    x: u32,
}

// Surrogate we can use instead of `SomeForeignType`
pub fn some_foreign_type() -> CType {
    let mut composite = CompositeType::new("SomeForeignType".to_string());
    composite.add_field(Field::new("x".to_string(), CType::Primitive(PrimitiveType::U32)));
    CType::Composite(composite)
}

#[ffi_type]
#[repr(C)]
pub struct Empty {}

#[ffi_type(opaque)]
pub struct Opaque {
    _internal: *const Vec3f32,
}

#[ffi_type]
#[repr(C)]
pub struct Generic<'a, T>
where
    T: 'static,
    T: CTypeInfo,
{
    pub x: &'a T,
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
pub struct Vec3f32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[ffi_type(surrogates(foreign1 = "some_foreign_type", foreign2 = "some_foreign_type"))]
#[repr(C)]
pub struct Container {
    pub foreign1: SomeForeignType,
    pub foreign2: SomeForeignType,
}

#[ffi_type(patterns(success_enum = "Ok"))]
#[repr(C)]
pub enum FFIError {
    Ok = 0,
    Null = 100,
    Fail = 200,
}

/// Documented enum.
#[ffi_type]
#[repr(C)]
pub enum EnumDocumented {
    /// Variant A.
    A,
    /// Variant B.
    B,
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
pub struct UseAsciiStringPattern<'a> {
    pub ascii_string: AsciiPointer<'a>,
}

// Used for the `context_init(**Context)`, `context_use(*Context)`,
// `context_destroy(**Context)` pattern that acts like constructors / methods / destructors.
#[ffi_type(opaque)]
#[repr(C)]
pub struct Context {
    pub(crate) some_field: u32,
}

// Doesn't need annotations.
pub type Callback = extern "C" fn(u8) -> u8;
