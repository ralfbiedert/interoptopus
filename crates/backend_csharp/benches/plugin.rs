use interoptopus::ffi;
use interoptopus::wire::Wire;
use interoptopus_csharp::plugin::DotNetRuntime;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, Instant};

fn plugin_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/plugins/dotnet_plugin/bin/Release/net10.0")
}

fn dll_path() -> PathBuf {
    let path = plugin_dir().join("Plugin.dll");
    assert!(path.exists(), "Plugin DLL not found at {path:?}. Run `dotnet build` in tests/plugins/dotnet_plugin/ first.");
    path
}

#[derive(Clone, Copy)]
#[ffi]
pub struct Big16 {
    pub f00: u32, pub f01: u32, pub f02: u32, pub f03: u32,
    pub f04: u32, pub f05: u32, pub f06: u32, pub f07: u32,
    pub f08: u32, pub f09: u32, pub f10: u32, pub f11: u32,
    pub f12: u32, pub f13: u32, pub f14: u32, pub f15: u32,
}

interoptopus::plugin!(BenchPlugin {
    fn primitive_void();
    fn primitive_u32(x: u32) -> u32;
    impl Foo {
        fn create() -> Self;
        fn wire(&self, x: Wire<String>) -> Wire<String>;
        fn wire2(&self, x: Wire<HashMap<String, String>>) -> Wire<HashMap<String, String>>;
        fn big16(&self, x: Big16) -> Big16;
    }
});

const ITERATIONS: u32 = 100_000;

fn calibrate() -> Duration {
    measure(ITERATIONS, || {})
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
    let loader = runtime
        .dll_loader_with_namespace(dll_path().to_str().unwrap(), "My.Company")
        .expect("Failed to load plugin DLL");
    let plugin = BenchPlugin::new(&loader).expect("Failed to load plugin");

    let baseline = calibrate();
    let mut entries: Vec<Entry> = Vec::new();

    let t = measure(ITERATIONS, || plugin.primitive_void());
    let ns = ns_per_call(t, baseline, ITERATIONS);
    println!("primitive_void(): {ns:.0}");
    entries.push(Entry { name: "primitive_void()".to_string(), ns });

    let t = measure(ITERATIONS, || {
        let _ = plugin.primitive_u32(42);
    });
    let ns = ns_per_call(t, baseline, ITERATIONS);
    println!("primitive_u32(42): {ns:.0}");
    entries.push(Entry { name: "primitive_u32(42)".to_string(), ns });

    let foo = plugin.foo_create();
    let hello = "hello world".to_string();
    let t = measure(ITERATIONS, || {
        let _ = foo.wire(Wire::from(hello.clone())).unwire();
    });
    let ns = ns_per_call(t, baseline, ITERATIONS);
    println!("foo.wire(Wire::from(\"hello world\")).unwire(): {ns:.0}");
    entries.push(Entry { name: r#"foo.wire(Wire::from("hello world")).unwire()"#.to_string(), ns });

    let map16: HashMap<String, String> = (0..16)
        .map(|i| (format!("{:016}", i), format!("{:016}", i)))
        .collect();
    let t = measure(ITERATIONS, || {
        let _ = foo.wire2(Wire::from(map16.clone())).unwire();
    });
    let ns = ns_per_call(t, baseline, ITERATIONS);
    println!("foo.wire2(Wire::from(map16x16)).unwire(): {ns:.0}");
    entries.push(Entry { name: "foo.wire2(Wire::from(16x{16char,16char})).unwire()".to_string(), ns });

    let big16 = Big16 {
        f00: 0, f01: 1, f02: 2, f03: 3,
        f04: 4, f05: 5, f06: 6, f07: 7,
        f08: 8, f09: 9, f10: 10, f11: 11,
        f12: 12, f13: 13, f14: 14, f15: 15,
    };
    let t = measure(ITERATIONS, || {
        let _ = foo.big16(big16);
    });
    let ns = ns_per_call(t, baseline, ITERATIONS);
    println!("foo.big16(big16): {ns:.0}");
    entries.push(Entry { name: "foo.big16(Big16 { 16x u32 })".to_string(), ns });

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
