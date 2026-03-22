use crate::{define_plugin, load_plugin};
use interoptopus::plugin;
use interoptopus_csharp::plugin::DotNetRuntime;
use reference_project::plugins::functions::{Behavior, Primitives};
use std::error::Error;
use std::panic::catch_unwind;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

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
    let exception_called = Arc::new(AtomicBool::new(false));
    let exception_called_clone = Arc::clone(&exception_called);

    let loader = DotNetRuntime::new()?
        .set_exception_handler(move |x| {
            println!("{x}");
            exception_called_clone.store(true, Ordering::SeqCst);
        })
        .dll_loader_with_namespace(super::plugin_path_for("functions_behavior.dll"), "My.Company")?;

    let plugin = Behavior::new(&loader)?;

    plugin.panic();

    assert!(exception_called.load(Ordering::SeqCst), "exception handler was not called after panic");

    Ok(())
}
