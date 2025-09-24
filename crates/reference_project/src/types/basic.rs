use interoptopus::ffi;

/// Empty structs are only allowed as opaques.
#[ffi(opaque)]
pub struct Empty {}

#[ffi]
pub struct Tupled(pub u8);

#[ffi]
#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3f32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Documented struct.
#[ffi]
pub struct StructDocumented {
    /// Documented field.
    pub x: f32,
}
