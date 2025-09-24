use crate::types::basic::Vec3f32;
use crate::types::complex::Layer3;
use crate::types::enums::EnumPayload;
use interoptopus::ffi;

#[ffi]
pub fn enums_1(_: EnumPayload) {}

#[ffi]
pub fn enums_2(x: EnumPayload) -> EnumPayload {
    match x {
        EnumPayload::A => x,
        EnumPayload::B(x) => EnumPayload::B(Vec3f32 { x: x.x * 2.0, y: x.y * 2.0, z: x.z * 2.0 }),
        EnumPayload::C(x) => EnumPayload::C(x * 2),
    }
}

#[ffi]
pub fn enums_3(x: &mut EnumPayload) -> &EnumPayload {
    match x {
        EnumPayload::A => (),
        EnumPayload::B(v) => *x = EnumPayload::B(Vec3f32 { x: v.x * 2.0, y: v.y * 2.0, z: v.z * 2.0 }),
        EnumPayload::C(v) => *x = EnumPayload::C(*v * 2),
    }

    x
}

#[ffi]
pub fn enums_4(x: Layer3<ffi::String>) -> ffi::String {
    match x {
        Layer3::A(x) => x.maybe_3,
        Layer3::B(x) => x.layer_1.maybe_3,
    }
}
