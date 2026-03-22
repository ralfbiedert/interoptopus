use interoptopus::wire::Wire;
use interoptopus_csharp::plugin::DotNetRuntime;
use reference_project::plugins::functions::Primitives;
use reference_project::plugins::service::{ServiceAsync, ServiceBasic};
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

fn calibrate() -> Duration {
    measure(ITERATIONS, || {})
}

fn measure_async<F, Fut>(rt: &tokio::runtime::Runtime, n: u32, f: F) -> Duration
where
    F: Fn() -> Fut,
    Fut: Future<Output = ()>,
{
    for _ in 0..n {
        rt.block_on(f());
    }
    let start = Instant::now();
    for _ in 0..n {
        rt.block_on(f());
    }
    start.elapsed()
}

fn calibrate_async(rt: &tokio::runtime::Runtime) -> Duration {
    measure_async(rt, ITERATIONS, || async {})
}

fn measure<F: Fn()>(n: u32, f: F) -> Duration {
    // warmup
    for _ in 0..n {
        f();
    }

    let start = Instant::now();
    for _ in 0..n {
        f();
    }
    start.elapsed()
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

fn main() {
    println!("Running plugin benchmarks (Rust → .NET) ...");

    let runtime = DotNetRuntime::new().expect("Failed to create .NET runtime");

    let primitives_loader = runtime
        .dll_loader_with_namespace(dll_path("functions_primitive.dll"), "My.Company")
        .expect("Failed to load functions_primitive.dll");
    let primitives = Primitives::new(&primitives_loader).expect("Failed to load Primitives plugin");

    let wire_loader = runtime
        .dll_loader_with_namespace(dll_path("wire.dll"), "My.Company")
        .expect("Failed to load wire.dll");
    let wired = Wired::new(&wire_loader).expect("Failed to load Wired plugin");

    let service_loader = runtime
        .dll_loader_with_namespace(dll_path("service_basic.dll"), "My.Company")
        .expect("Failed to load service_basic.dll");
    let service = ServiceBasic::new(&service_loader).expect("Failed to load ServiceBasic plugin");

    let service_async_loader = runtime
        .dll_loader_with_namespace(dll_path("service_async.dll"), "My.Company")
        .expect("Failed to load service_async.dll");
    let service_async = ServiceAsync::new(&service_async_loader).expect("Failed to load ServiceAsync plugin");

    let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");

    let baseline = calibrate();
    let baseline_async = calibrate_async(&rt);
    let mut entries: Vec<Entry> = Vec::new();

    let t = measure(ITERATIONS, || primitives.primitive_void());
    let ns = ns_per_call(t, baseline, ITERATIONS);
    println!("primitive_void(): {ns:.0}");
    entries.push(Entry { name: "primitive_void()".to_string(), ns });

    let t = measure(ITERATIONS, || {
        let _ = primitives.primitive_u32(42);
    });
    let ns = ns_per_call(t, baseline, ITERATIONS);
    println!("primitive_u32(42): {ns:.0}");
    entries.push(Entry { name: "primitive_u32(42)".to_string(), ns });

    let svc = service.servicea_create();
    let t = measure(ITERATIONS, || {
        let _ = svc.call(5);
    });
    let ns = ns_per_call(t, baseline, ITERATIONS);
    println!("svc.call(5): {ns:.0}");
    entries.push(Entry { name: "svc.call(5)".to_string(), ns });

    let json_input = "{}".to_string();
    let t = measure(ITERATIONS, || {
        let _ = wired.wire_string(Wire::from(json_input.clone())).unwire();
    });
    let ns = ns_per_call(t, baseline, ITERATIONS);
    println!("wire_string(Wire::from(\"{{}}\")).unwire(): {ns:.0}");
    entries.push(Entry { name: r#"wire_string(Wire::from("{}")).unwire()"#.to_string(), ns });

    let map1: HashMap<String, String> = (0..1).map(|i| (format!("{:016}", i), format!("{:016}", i))).collect();
    let t = measure(ITERATIONS, || {
        let _ = wired.wire_hashmap_string(Wire::from(map1.clone())).unwire();
    });
    let ns = ns_per_call(t, baseline, ITERATIONS);
    println!("wire_hashmap_string(Wire::from(1x{{1char,1char}})).unwire(): {ns:.0}");
    entries.push(Entry { name: "wire_hashmap_string(Wire::from(16x{16char,16char})).unwire()".to_string(), ns });

    let map16: HashMap<String, String> = (0..16).map(|i| (format!("{:016}", i), format!("{:016}", i))).collect();
    dbg!(&map16);
    let t = measure(ITERATIONS, || {
        let _ = wired.wire_hashmap_string(Wire::from(map16.clone())).unwire();
    });
    let ns = ns_per_call(t, baseline, ITERATIONS);
    println!("wire_hashmap_string(Wire::from(16x{{16char,16char}})).unwire(): {ns:.0}");
    entries.push(Entry { name: "wire_hashmap_string(Wire::from(16x{16char,16char})).unwire()".to_string(), ns });

    let hashmap = HashMap::from([("foo".to_string(), "bar".to_string())]);

    let t = measure_async(&rt, ITERATIONS, || async {
        let _ = service_async.add_one(1).await;
    });
    let ns = ns_per_call(t, baseline_async, ITERATIONS);
    println!("async service_async.add_one(1): {ns:.0}");
    entries.push(Entry { name: "async service_async.add_one(1)".to_string(), ns });

    let t = measure_async(&rt, ITERATIONS, || async {
        let wire = Wire::from(hashmap.clone());
        let _ = service_async.wire_1(wire).await;
    });
    let ns = ns_per_call(t, baseline_async, ITERATIONS);
    println!("async service_async.wire_1(Wire::from({{1char,1char}})).await: {ns:.0}");
    entries.push(Entry { name: r#"async service_async.wire_1(Wire::from({1char,1char})).await"#.to_string(), ns });

    let async_svc = service_async.asyncbasic_create();

    let t = measure_async(&rt, ITERATIONS, || async {
        let _ = async_svc.add_one(1).await;
    });
    let ns = ns_per_call(t, baseline_async, ITERATIONS);
    println!("async async_svc.add_one(1): {ns:.0}");
    entries.push(Entry { name: "async async_svc.add_one(1)".to_string(), ns });

    let t = measure_async(&rt, ITERATIONS, || async {
        let wire = Wire::from(hashmap.clone());
        let _ = async_svc.wire_1(wire).await;
    });
    let ns = ns_per_call(t, baseline_async, ITERATIONS);
    println!("async async_svc.wire_1(Wire::from({{1char,1char}})).await: {ns:.0}");
    entries.push(Entry { name: r#"async async_svc.wire_1(Wire::from({1char,1char})).await"#.to_string(), ns });

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
}
