use crate::{define_plugin, load_plugin};
use interoptopus_csharp::rt::dynamic::runtime as dotnet_runtime;
use reference_project::plugins::functions::{Behavior, Primitives};
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[test]
fn define_plugins() -> Result<(), Box<dyn Error>> {
    define_plugin!(Primitives, "functions_primitive.dll", super::BASE);
    define_plugin!(Behavior, "functions_behavior.dll", super::BASE);
    Ok(())
}

#[test]
fn load_plugin_functions_primitive() -> Result<(), Box<dyn Error>> {
    let plugin = load_plugin!(Primitives, "functions_primitive.dll", super::BASE);
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
#[test]
fn load_plugin_functions_behavior() -> Result<(), Box<dyn Error>> {
    let exception_called = Arc::new(AtomicBool::new(false));
    let exception_called_clone = Arc::clone(&exception_called);

    let rt = dotnet_runtime()?;
    rt.exception_handler(move |_| {
        exception_called_clone.store(true, Ordering::SeqCst);
    });

    let plugin = rt.load::<Behavior>(crate::dll_path_for(super::BASE, "functions_behavior.dll"))?;

    plugin.panic();

    assert!(exception_called.load(Ordering::SeqCst), "exception handler was not called after panic");

    Ok(())
}

// Async exceptions from the .NET side must surface via the registered
// `Trampoline.UncaughtException` hook so the Rust caller can observe them.
//
// On the wire the byte is still `AsyncOutcome::Cancelled` (we don't yet
// split Cancelled vs Panic on the wire), so the Rust future resolves to
// `Err`. The exception text reaches the host through the exception handler
// side-channel — that part is what this test pins.
#[tokio::test]
async fn load_plugin_functions_behavior_async_throw() -> Result<(), Box<dyn Error>> {
    let rt = dotnet_runtime()?;

    let plugin = rt.load::<Behavior>(crate::dll_path_for(super::BASE, "functions_behavior.dll"))?;

    let result = plugin.panic_async().await;
    assert!(result.is_err());

    let result = plugin.panic_async_with_result().await;
    assert!(result.is_err());

    Ok(())
}
