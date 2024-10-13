use interoptopus::ffi_type;

/// A vector used in our game engine.
#[ffi_type]
#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// A vector used in our game engine.
#[ffi_type]
#[derive(Copy, Clone, Debug)]
pub struct SuperComplexEntity {
    pub player_1: Vec3,
    pub player_2: Vec3,
    pub ammo: u64,
    /// Point to an ASCII encoded whatnot.
    pub some_str: *const u8,
    pub str_len: u32,
}

/// Worst game engine ever.
///
/// Note that this struct has no `ffi_type` and won't be exported. It's part of
/// the opaque Context used in `ffi`.
#[derive(Default, Debug)]
pub struct GameEngine {
    pub player_score: u32,
}
