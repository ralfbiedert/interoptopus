use crate::types::basic::Vec3f32;
use crate::types::enums::EnumPayload;
use interoptopus::ffi_function;

#[ffi_function]
pub fn enums_1(_: EnumPayload) {}

#[ffi_function]
pub fn enums_2(x: EnumPayload) -> EnumPayload {
    match x {
        EnumPayload::A => x,
        EnumPayload::B(x) => EnumPayload::B(Vec3f32 { x: x.x * 2.0, y: x.y * 2.0, z: x.z * 2.0 }),
        EnumPayload::C(x) => EnumPayload::C(x * 2),
    }
}

#[ffi_function]
pub fn enums_3(x: &mut EnumPayload) -> &EnumPayload {
    match x {
        EnumPayload::A => (),
        EnumPayload::B(v) => *x = EnumPayload::B(Vec3f32 { x: v.x * 2.0, y: v.y * 2.0, z: v.z * 2.0 }),
        EnumPayload::C(v) => *x = EnumPayload::C(*v * 2),
    }

    x
}
