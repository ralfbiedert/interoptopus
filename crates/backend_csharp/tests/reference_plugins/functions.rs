use crate::{define_plugin, load_plugin};
use interoptopus::plugin;
use reference_project::plugins::functions::{Behavior, Primitives};
use std::error::Error;
use std::panic::catch_unwind;

#[test]
fn define_plugins() -> Result<(), Box<dyn Error>> {
    define_plugin!(Primitives, "functions_primitive.dll");
    define_plugin!(Behavior, "functions_behavior.dll");
    Ok(())
}

// Test ignored since we can't rely on a working .NET runtime being available on CI
#[ignore]
#[test]
fn load_plugin_functions_primitive() -> Result<(), Box<dyn Error>> {
    let plugin = load_plugin!(Primitives, "functions_primitive.dll");

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

// Test ignored since we can't rely on a working .NET runtime being available on CI
#[ignore]
#[test]
fn load_plugin_functions_behavior() -> Result<(), Box<dyn Error>> {
    let plugin = load_plugin!(Behavior, "functions_behavior.dll");

    // As C# apparently doesn't support `C-unwind` when called this way
    // it will crash the application. That means either make sure
    // - your methods don't throw,
    // - you add a manual try {} catch {} around them and decide what to return if it throws,
    // - use ffi::Result
    // plugin.panic();

    Ok(())
}
