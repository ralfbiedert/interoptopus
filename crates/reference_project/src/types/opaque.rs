use crate::types::basic::Vec3f32;
use interoptopus::ffi_type;

#[ffi_type(opaque)]
#[allow(dead_code)]
pub struct Opaque {
    _internal: *const Vec3f32,
    _unused: (),
}

/// This can also be used for the `class` pattern.
#[ffi_type(opaque)]
#[allow(unused)]
pub struct SomeContext {
    pub(crate) some_field: u32,
}
