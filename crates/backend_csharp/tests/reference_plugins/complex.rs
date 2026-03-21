use crate::{define_plugin, load_plugin};
use reference_project::plugins::complex::Complex;
use reference_project::types::basic::Vec3f32;
use reference_project::types::enums::EnumPayload;
use std::error::Error;

#[test]
fn define_plugin() -> Result<(), Box<dyn Error>> {
    define_plugin!(Complex, "complex.dll");
    Ok(())
}

// Test ignored since we can't rely on a working .NET runtime being available on CI
#[ignore]
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

    Ok(())
}
