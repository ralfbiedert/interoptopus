use crate::define_check_and_load_plugin;
use reference_project::plugins::basic::Primitives;
use std::error::Error;

#[ignore]
#[test]
fn basic() -> Result<(), Box<dyn Error>> {
    let plugin = define_check_and_load_plugin!(Primitives, "basic.dll");

    plugin.primitive_void();
    plugin.primitive_u32(0);
    Ok(())
}
