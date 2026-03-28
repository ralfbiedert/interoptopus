use interoptopus::telemetry::Metrics;
use interoptopus::wire::Wire;
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

fn measure<F: Fn()>(n: u32, f: F) -> Vec<Duration> {
    for _ in 0..n {
        f();
    }
    let mut times = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let start = Instant::now();
        f();
        times.push(start.elapsed());
    }
    times
}

fn measure_async<F, Fut>(rt: &tokio::runtime::Runtime, n: u32, f: F) -> Vec<Duration>
where
    F: Fn() -> Fut,
    Fut: Future<Output = ()>,
{
    rt.block_on(async {
        for _ in 0..n {
            f().await;
        }
    });
    rt.block_on(async {
        let mut times = Vec::with_capacity(n as usize);
        for _ in 0..n {
            let start = Instant::now();
            f().await;
            times.push(start.elapsed());
        }
        times
    })
}

fn measure_async_parallel<F, Fut>(rt: &tokio::runtime::Runtime, n: u32, parallelism: usize, f: F) -> Vec<Duration>
where
    F: Fn() -> Fut,
    Fut: Future<Output = ()> + Send + 'static,
{
    // warmup
    rt.block_on(async {
        let mut set = tokio::task::JoinSet::new();
        for _ in 0..n {
            set.spawn(f());
            if set.len() >= parallelism {
                let _ = set.join_next().await;
            }
        }
        while set.join_next().await.is_some() {}
    });
    // measure: each batch of `parallelism` futures is timed as a wall-clock block
    rt.block_on(async {
        let mut times = Vec::with_capacity(n as usize);
        let mut remaining = n;
        while remaining > 0 {
            let batch = (parallelism as u32).min(remaining) as usize;
            let mut set = tokio::task::JoinSet::new();
            for _ in 0..batch {
                set.spawn(f());
            }
            let start = Instant::now();
            while set.join_next().await.is_some() {}
            let elapsed = start.elapsed() / batch as u32;
            for _ in 0..batch {
                times.push(elapsed);
            }
            remaining -= batch as u32;
        }
        times
    })
}

fn baseline_median(mut times: Vec<Duration>) -> Duration {
    times.sort_unstable();
    times[times.len() / 2]
}

fn median_ns(times: &mut Vec<Duration>, baseline: Duration) -> f64 {
    times.sort_unstable();
    let t = times[times.len() / 2].as_nanos() as f64;
    let b = baseline.as_nanos() as f64;
    (t - b).max(0.0)
}

struct Entry {
    name: String,
    ns: f64,
}

fn main() {
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

    let tokio_rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");

    let baseline = baseline_median(measure(ITERATIONS, || {}));
    let baseline_async = baseline_median(measure_async(&tokio_rt, ITERATIONS, || async {}));
    let mut entries: Vec<Entry> = Vec::new();

    let ns = median_ns(&mut measure(ITERATIONS, || primitives.primitive_void()), baseline);
    println!("primitive_void(): {ns:.0}");
    entries.push(Entry { name: "primitive_void()".to_string(), ns });

    let ns = median_ns(&mut measure(ITERATIONS, || { let _ = primitives.primitive_u32(42); }), baseline);
    println!("primitive_u32(42): {ns:.0}");
    entries.push(Entry { name: "primitive_u32(42)".to_string(), ns });

    let svc = service.service_a_create();
    let ns = median_ns(&mut measure(ITERATIONS, || { let _ = svc.call(5); }), baseline);
    println!("svc.call(5): {ns:.0}");
    entries.push(Entry { name: "svc.call(5)".to_string(), ns });

    let json_input = "{}".to_string();
    let ns = median_ns(&mut measure(ITERATIONS, || { let _ = wired.wire_string(Wire::from(json_input.clone())).unwire(); }), baseline);
    println!(r#"wire_string(Wire::from("{{}}")).unwire(): {ns:.0}"#);
    entries.push(Entry { name: r#"wire_string(Wire::from("{}")).unwire()"#.to_string(), ns });

    let map1: HashMap<String, String> = (0..1).map(|i| (format!("{:016}", i), format!("{:016}", i))).collect();
    let ns = median_ns(&mut measure(ITERATIONS, || { let _ = wired.wire_hashmap_string(Wire::from(map1.clone())).unwire(); }), baseline);
    println!("wire_hashmap_string(Wire::from(1x{{16char,16char}})).unwire(): {ns:.0}");
    entries.push(Entry { name: "wire_hashmap_string(Wire::from(1x{16char,16char})).unwire()".to_string(), ns });

    let map16: HashMap<String, String> = (0..16).map(|i| (format!("{:016}", i), format!("{:016}", i))).collect();
    let ns = median_ns(&mut measure(ITERATIONS, || { let _ = wired.wire_hashmap_string(Wire::from(map16.clone())).unwire(); }), baseline);
    println!("wire_hashmap_string(Wire::from(16x{{16char,16char}})).unwire(): {ns:.0}");
    entries.push(Entry { name: "wire_hashmap_string(Wire::from(16x{16char,16char})).unwire()".to_string(), ns });

    let hashmap = HashMap::from([("foo".to_string(), "bar".to_string())]);

    let service_async = std::sync::Arc::new(service_async);

    let svc2 = service_async.clone();
    let ns = median_ns(&mut measure_async(&tokio_rt, ITERATIONS, move || { let s = svc2.clone(); async move { let _ = s.add_one(1).await; } }), baseline_async);
    println!("async service_async.add_one(1) [sequential]: {ns:.0}");
    entries.push(Entry { name: "async service_async.add_one(1) [sequential]".to_string(), ns });

    let svc2 = service_async.clone();
    let ns = median_ns(&mut measure_async_parallel(&tokio_rt, ITERATIONS, 64, move || { let s = svc2.clone(); async move { let _ = s.add_one(1).await; } }), baseline_async);
    println!("async service_async.add_one(1) [64 in-flight]: {ns:.0}");
    entries.push(Entry { name: "async service_async.add_one(1) [64 in-flight]".to_string(), ns });

    let svc2 = service_async.clone();
    let hm = hashmap.clone();
    let ns = median_ns(&mut measure_async(&tokio_rt, ITERATIONS, move || { let s = svc2.clone(); let wire = Wire::from(hm.clone()); async move { let _ = s.wire_1(wire).await; } }), baseline_async);
    println!("async service_async.wire_1(Wire::from({{1char,1char}})).await: {ns:.0}");
    entries.push(Entry { name: r#"async service_async.wire_1(Wire::from({1char,1char})).await"#.to_string(), ns });

    let async_svc = std::sync::Arc::new(service_async.async_basic_create());

    let svc2 = async_svc.clone();
    let ns = median_ns(&mut measure_async(&tokio_rt, ITERATIONS, move || { let s = svc2.clone(); async move { let _ = s.add_one(1).await; } }), baseline_async);
    println!("async async_svc.add_one(1) [sequential]: {ns:.0}");
    entries.push(Entry { name: "async async_svc.add_one(1) [sequential]".to_string(), ns });

    let svc2 = async_svc.clone();
    let ns = median_ns(&mut measure_async_parallel(&tokio_rt, ITERATIONS, 64, move || { let s = svc2.clone(); async move { let _ = s.add_one(1).await; } }), baseline_async);
    println!("async async_svc.add_one(1) [64 in-flight]: {ns:.0}");
    entries.push(Entry { name: "async async_svc.add_one(1) [64 in-flight]".to_string(), ns });

    let svc2 = async_svc.clone();
    let hm = hashmap.clone();
    let ns = median_ns(&mut measure_async(&tokio_rt, ITERATIONS, move || { let s = svc2.clone(); let wire = Wire::from(hm.clone()); async move { let _ = s.wire_1(wire).await; } }), baseline_async);
    println!("async async_svc.wire_1(Wire::from({{1char,1char}})).await: {ns:.0}");
    entries.push(Entry { name: r#"async async_svc.wire_1(Wire::from({1char,1char})).await"#.to_string(), ns });

    // Write markdown results
    let mut md = String::new();
    md.push_str("# Plugin Call Overheads (Rust → .NET)\n\n");
    md.push_str("Times were determined by running the construct 100k times (warmup + measure), ");
    md.push_str("reporting the median ns per call with an empty-loop baseline median subtracted.\n\n");
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
