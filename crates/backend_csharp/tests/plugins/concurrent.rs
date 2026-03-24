use crate::plugins::plugin_path_for;
use interoptopus_csharp::plugin::DotNetRuntime;
use reference_project::plugins::functions::Behavior;
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// Test ignored since we can't rely on a working .NET runtime being available on CI
#[test]
fn multiple_concurrent_plugins_work() -> Result<(), Box<dyn Error>> {
    let handles: Vec<_> = (0..16)
        .map(|_| {
            std::thread::spawn(move || {
                let exception_called = Arc::new(AtomicBool::new(false));
                let exception_called_clone = Arc::clone(&exception_called);

                let runtime = DotNetRuntime::new().expect("failed to create runtime").exception_handler(move |_| {
                    exception_called_clone.store(true, Ordering::SeqCst);
                });

                let loader = runtime.dll_loader(plugin_path_for("functions_behavior.dll")).expect("failed to load dll");

                let plugin = Behavior::new(&loader).expect("failed to create plugin");
                plugin.panic();

                assert!(exception_called.load(Ordering::SeqCst), "exception handler was not called after panic");
            })
        })
        .collect();

    for h in handles {
        h.join().expect("thread panicked");
    }

    Ok(())
}
