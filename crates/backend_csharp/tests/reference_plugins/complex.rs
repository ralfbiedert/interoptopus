use crate::{define_plugin, load_plugin};
use interoptopus::lang::types::Primitive::U16;
use reference_project::plugins::complex::Complex;
use reference_project::types::arrays::{Array, NestedArray};
use reference_project::types::basic::Vec3f32;
use reference_project::types::enums::{EnumPayload, EnumRenamedXYZ};
use std::error::Error;

#[test]
fn define_plugin() -> Result<(), Box<dyn Error>> {
    define_plugin!(Complex, "complex.dll");
    Ok(())
}

#[test]
fn load_plugin() -> Result<(), Box<dyn Error>> {
    let plugin = load_plugin!(Complex, "complex.dll");

    let input = Vec3f32 { x: 1.0, y: 2.0, z: 3.0 };
    let result = plugin.vec3f32(input);
    assert_eq!(result.x, 2.0);
    assert_eq!(result.y, 1.0);
    assert_eq!(result.z, 3.0);

    let result = plugin.enum_payload(EnumPayload::A);
    assert!(matches!(result, EnumPayload::B(v) if v.x == 1.0 && v.y == 2.0 && v.z == 3.0));

    let array = NestedArray {
        field_enum: EnumRenamedXYZ::X,
        field_vec: Default::default(),
        field_bool: false,
        field_int: 0,
        field_array: [0_u16; 5],
        field_array_2: [0_u16; 5],
        field_struct: Array { data: [0_u8; 16] },
    };
    let array = plugin.nested_array(array);
    assert_eq!(array.field_array[1], 2);

    Ok(())
}
