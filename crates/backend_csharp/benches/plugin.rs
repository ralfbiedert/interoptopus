use interoptopus::telemetry::Metrics;
use interoptopus::wire::Wire;
use reference_project::plugins::service::{ServiceAsync, ServiceBasic};
use reference_project::plugins::functions::Primitives;
use reference_project::plugins::wire::Wired;
use std::collections::HashMap;
use std::future::Future;
use std::path::PathBuf;
use std::time::{Duration, Instant};

fn plugins_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/reference_plugins/_plugins")
}

fn dll_path(name: &str) -> PathBuf {
    let path = plugins_dir().join(name);
    assert!(path.exists(), "Plugin DLL not found at {path:?}. Run `just build-dotnet-plugins` first.");
    path
}

const ITERATIONS: u32 = 100_000;

fn measure<F: Fn()>(n: u32, f: F) -> Duration {
    for _ in 0..n { f(); }
    let start = Instant::now();
    for _ in 0..n { f(); }
    start.elapsed()
}

async fn measure_async<F, Fut>(n: u32, f: F) -> Duration
where
    F: Fn() -> Fut,
    Fut: Future,
{
    for _ in 0..n { f().await; }
    let start = Instant::now();
    for _ in 0..n { f().await; }
    start.elapsed()
}

fn calibrate() -> Duration {
    measure(ITERATIONS, || {})
}

fn ns_per_call(elapsed: Duration, baseline: Duration, n: u32) -> f64 {
    let total_ns = elapsed.as_nanos() as f64;
    let base_ns = baseline.as_nanos() as f64;
    (total_ns - base_ns).max(0.0) / f64::from(n)
}

struct Entry {
    name: String,
    ns: f64,
}

#[tokio::main]
async fn main() {
    println!("Running plugin benchmarks (Rust → .NET) ...");

    let rt = interoptopus_csharp::rt::dynamic::runtime().expect("Failed to create .NET runtime");

    let primitives = rt.load::<Primitives>(dll_path("functions_primitive.dll")).expect("Failed to load Primitives plugin");
    let wired = rt.load::<Wired>(dll_path("wire.dll")).expect("Failed to load Wired plugin");
    let service = rt.load::<ServiceBasic>(dll_path("service_basic.dll")).expect("Failed to load ServiceBasic plugin");
    let service_async = rt.load::<ServiceAsync>(dll_path("service_async.dll")).expect("Failed to load ServiceAsync plugin");

    primitives.metrics_enable(true);
    wired.metrics_enable(true);
    service.metrics_enable(true);
    service_async.metrics_enable(true);

    let baseline = calibrate();
    let mut entries: Vec<Entry> = Vec::new();

    // Sync benchmarks
    let t = measure(ITERATIONS, || primitives.primitive_void());
    entries.push(Entry { name: "primitive_void()".to_string(), ns: ns_per_call(t, baseline, ITERATIONS) });

    let t = measure(ITERATIONS, || { let _ = primitives.primitive_u32(42); });
    entries.push(Entry { name: "primitive_u32(42)".to_string(), ns: ns_per_call(t, baseline, ITERATIONS) });

    let svc = service.service_a_create();
    let t = measure(ITERATIONS, || { let _ = svc.call(5); });
    entries.push(Entry { name: "svc.call(5)".to_string(), ns: ns_per_call(t, baseline, ITERATIONS) });

    let json_input = "{}".to_string();
    let t = measure(ITERATIONS, || { let _ = wired.wire_string(Wire::from(json_input.clone())).unwire(); });
    entries.push(Entry { name: r#"wire_string(Wire::from("{}")).unwire()"#.to_string(), ns: ns_per_call(t, baseline, ITERATIONS) });

    let map16: HashMap<String, String> = (0..16).map(|i| (format!("{:016}", i), format!("{:016}", i))).collect();
    let t = measure(ITERATIONS, || { let _ = wired.wire_hashmap_string(Wire::from(map16.clone())).unwire(); });
    entries.push(Entry { name: "wire_hashmap_string(Wire::from(16x{16char,16char})).unwire()".to_string(), ns: ns_per_call(t, baseline, ITERATIONS) });

    // Async benchmarks
    let t = measure_async(ITERATIONS, || service_async.call_void()).await;
    entries.push(Entry { name: "async call_void()".to_string(), ns: ns_per_call(t, baseline, ITERATIONS) });

    let t = measure_async(ITERATIONS, || service_async.add_one(1)).await;
    entries.push(Entry { name: "async add_one(1)".to_string(), ns: ns_per_call(t, baseline, ITERATIONS) });

    let t = measure_async(ITERATIONS, || service_async.wire_1(Wire::from(map16.clone()))).await;
    entries.push(Entry { name: "async wire_1(16x{16char,16char})".to_string(), ns: ns_per_call(t, baseline, ITERATIONS) });

    let async_svc = service_async.async_basic_create();
    let t = measure_async(ITERATIONS, || async_svc.call_void()).await;
    entries.push(Entry { name: "async_svc.call_void()".to_string(), ns: ns_per_call(t, baseline, ITERATIONS) });

    let t = measure_async(ITERATIONS, || async_svc.add_one(1)).await;
    entries.push(Entry { name: "async_svc.add_one(1)".to_string(), ns: ns_per_call(t, baseline, ITERATIONS) });

    let t = measure_async(ITERATIONS, || async_svc.wire_1(Wire::from(map16.clone()))).await;
    entries.push(Entry { name: "async_svc.wire_1(16x{16char,16char})".to_string(), ns: ns_per_call(t, baseline, ITERATIONS) });

    // Write markdown results
    let mut md = String::new();
    md.push_str("# Plugin Call Overheads (Rust → .NET)\n\n");
    md.push_str("Times were determined by running the construct 100k times (warmup + measure), ");
    md.push_str("computing ns per call with an empty-loop baseline subtracted.\n\n");
    md.push_str("| Construct | ns per call |\n");
    md.push_str("| --- | --- |\n");
    for e in &entries {
        md.push_str(&format!("| `{}` | {:.0} |\n", e.name, e.ns));
    }

    let results_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("benches/PLUGIN_RESULTS.md");
    std::fs::write(&results_path, &md).expect("failed to write PLUGIN_RESULTS.md");
    println!("\nResults written to {}", results_path.display());

    primitives.metrics_report().print_stdout();
    wired.metrics_report().print_stdout();
    service.metrics_report().print_stdout();
    service_async.metrics_report().print_stdout();
}
