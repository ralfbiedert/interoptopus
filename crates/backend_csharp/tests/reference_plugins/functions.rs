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
#[ignore]
// This test sets the `exception_handler`, which is a global singleton. We can only ever have one,
// so all tests relying on that are disabled unless we actively want to test them.
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

    // Sync fn returning a typed ffi::Result that throws inside must fold into ffi::Result::Panic
    // (handled by the generated `FromCallResult` wrapper), NOT surface through the outer Rust panic.
    let result = plugin.panic_with_result();
    assert!(matches!(result, interoptopus::ffi::Result::Panic), "typed sync throw must fold into ffi::Result::Panic, got {result:?}");

    Ok(())
}

// Async exceptions from the .NET side must surface to the Rust caller.
//
// - For async fns *without* a typed `ffi::Result` rval, the exception still folds
//   into the outer `Err(AsyncCancelled)` channel (Task is cancelled on the wire).
// - For async fns *with* a typed `ffi::Result<T, E>` rval, the exception is folded
//   into `ffi::Result::Panic` (discriminant 2) by the generated `FromCallResultAsync`
//   wrapper. The outer await resolves to `Ok(ffi::Result::Panic)`.
#[tokio::test]
#[ignore]
async fn load_plugin_functions_behavior_async_throw() -> Result<(), Box<dyn Error>> {
    let rt = dotnet_runtime()?;

    let plugin = rt.load::<Behavior>(crate::dll_path_for(super::BASE, "functions_behavior.dll"))?;

    let result = plugin.panic_async().await;
    assert!(result.is_err(), "untyped async throw should surface as outer Err(AsyncCancelled)");

    let inner = plugin.panic_async_with_result().await.expect("typed async throw should NOT be Err(AsyncCancelled)");
    assert!(matches!(inner, interoptopus::ffi::Result::Panic), "typed async throw must fold into ffi::Result::Panic, got {inner:?}");

    Ok(())
}
