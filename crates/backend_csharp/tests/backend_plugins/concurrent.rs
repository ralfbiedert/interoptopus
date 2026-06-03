use interoptopus_csharp::rt::dynamic::runtime as dotnet_runtime;
use reference_project::plugins::functions::Behavior;
use std::error::Error;
use std::panic::catch_unwind;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

const BASE: &str = "tests/reference_plugins";

#[test]
fn concurrent_same_plugins_work() -> Result<(), Box<dyn Error>> {
    // Each call to `plugin.panic()` triggers a C# exception that is folded into a
    // Rust panic on the calling thread via the per-thread uncaught-exception slot.
    // Catch each panic explicitly and count them.
    let panic_count = Arc::new(AtomicU32::new(0));

    let _rt = dotnet_runtime()?;

    let num_threads = 16;
    let handles: Vec<_> = (0..num_threads)
        .map(|_| {
            let panic_count = Arc::clone(&panic_count);
            std::thread::spawn(move || {
                let rt = dotnet_runtime().expect("failed to initialize .NET runtime");
                let plugin = rt
                    .load::<Behavior>(crate::dll_path_for(BASE, "functions_behavior.dll"))
                    .expect("failed to load plugin");

                let result = catch_unwind(std::panic::AssertUnwindSafe(|| plugin.panic()));
                if result.is_err() {
                    panic_count.fetch_add(1, Ordering::SeqCst);
                }
            })
        })
        .collect();

    for h in handles {
        h.join().expect("thread panicked");
    }

    assert_eq!(panic_count.load(Ordering::SeqCst), num_threads, "expected one panic per thread");

    Ok(())
}
