use crate::{define_plugin, load_plugin};
use interoptopus::plugin;
use reference_project::plugins::primitive::Primitives;
use std::error::Error;

#[test]
fn define_plugin() -> Result<(), Box<dyn Error>> {
    define_plugin!(Primitives, "basic.dll");
    Ok(())
}

// Test ignored since we can't rely on a working .NET runtime being available on CI
#[ignore]
#[test]
fn load_plugin() -> Result<(), Box<dyn Error>> {
    let plugin = load_plugin!(Primitives, "basic.dll");

    plugin.primitive_void();
    assert_eq!(plugin.primitive_u8(1), 2);
    assert_eq!(plugin.primitive_u16(1), 2);
    assert_eq!(plugin.primitive_u32(1), 2);
    assert_eq!(plugin.primitive_u64(1), 2);
    assert_eq!(plugin.primitive_i8(1), 2);
    assert_eq!(plugin.primitive_i16(1), 2);
    assert_eq!(plugin.primitive_i32(1), 2);
    assert_eq!(plugin.primitive_i64(1), 2);
    assert_eq!(plugin.primitive_f32(1.0), 2.0);
    assert_eq!(plugin.primitive_f64(1.0), 2.0);
    Ok(())
}
