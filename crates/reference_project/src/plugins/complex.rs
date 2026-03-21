use crate::types::basic::Vec3f32;
use crate::types::enums::EnumPayload;

interoptopus::plugin!(Complex {
    fn vec3f32(nested: Vec3f32) -> Vec3f32;
    fn enum_payload(nested: EnumPayload) -> EnumPayload;
    // This has issues with needed generated `.Unmanaged` conversion
    // fn nested_array(nested: NestedArray) -> NestedArray;
});
