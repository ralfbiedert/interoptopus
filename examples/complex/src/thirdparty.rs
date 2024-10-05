/// A type that may not be in your crate for which you can't `#[ffi_type]` or derive `CType`
/// but which you want to use in your ffi signatures nonetheless.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
pub struct ThirdPartyVecF32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[allow(unused)]
pub struct f32x4([f32; 4]);
