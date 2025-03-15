use interoptopus::ffi_type;

/// Empty structs are only allowed as opaques.
#[ffi_type(opaque)]
pub struct Empty {}

#[ffi_type]
pub struct Tupled(pub u8);

#[ffi_type]
#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3f32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Documented struct.
#[ffi_type]
pub struct StructDocumented {
    /// Documented field.
    pub x: f32,
}
