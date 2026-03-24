use crate::{define_plugin, load_plugin};
use interoptopus::ffi;
use reference_project::patterns::callback::MyCallback;
use reference_project::patterns::result::Error;
use reference_project::plugins::pattern::Pattern;
use reference_project::types::basic::Vec3f32;
use std::error::Error as StdError;

#[test]
fn define_plugin() -> Result<(), Box<dyn StdError>> {
    define_plugin!(Pattern, "pattern.dll");
    Ok(())
}

#[test]
fn load_plugin() -> Result<(), Box<dyn StdError>> {
    let plugin = load_plugin!(Pattern, "pattern.dll");

    // Plugin ignores input and always returns Ok(Vec3f32::default())
    let input = ffi::Result::Ok(Vec3f32 { x: 1.0, y: 2.0, z: 3.0 });
    let result = plugin.result(input).unwrap();
    assert_eq!(result.x, 0.0);
    assert_eq!(result.y, 0.0);
    assert_eq!(result.z, 0.0);

    // Also holds for Err input
    let err_input = ffi::Result::Err(Error::Fail);
    let result = plugin.result(err_input).unwrap();
    assert_eq!(result.x, 0.0);
    assert_eq!(result.y, 0.0);
    assert_eq!(result.z, 0.0);

    // Plugin ignores input and always returns Ok(Vec3f32::default())
    let callback = MyCallback::from_fn(|x| x + 1);
    let result = plugin.delegate_1(callback);
    let i = result.call(3, 4);
    assert_eq!(i, 8);

    Ok(())
}
