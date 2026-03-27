use interoptopus_csharp::rt::dynamic::runtime as dotnet_runtime;
use reference_project::plugins::functions::Behavior;
use std::error::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

const BASE: &str = "tests/reference_plugins";

#[test]
fn concurrent_same_plugins_work() -> Result<(), Box<dyn Error>> {
    let exception_count = Arc::new(AtomicU32::new(0));
    let exception_count_clone = Arc::clone(&exception_count);

    let rt = dotnet_runtime()?;
    rt.exception_handler(move |_| {
        exception_count_clone.fetch_add(1, Ordering::SeqCst);
    });

    let num_threads = 16;
    let handles: Vec<_> = (0..num_threads)
        .map(|_| {
            std::thread::spawn(move || {
                let rt = dotnet_runtime().expect("failed to initialize .NET runtime");
                let plugin = rt
                    .load::<Behavior>(crate::dll_path_for(BASE, "functions_behavior.dll"))
                    .expect("failed to load plugin");

                plugin.panic();
            })
        })
        .collect();

    for h in handles {
        h.join().expect("thread panicked");
    }

    assert_eq!(exception_count.load(Ordering::SeqCst), num_threads, "expected one exception per thread");

    Ok(())
}
