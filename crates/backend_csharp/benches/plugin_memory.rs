use memory_stats::memory_stats;
use reference_project::plugins::functions::Primitives;
use std::path::PathBuf;

fn plugins_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/reference_plugins/_plugins")
}

fn dll_path(name: &str) -> PathBuf {
    let path = plugins_dir().join(name);
    assert!(path.exists(), "Plugin DLL not found at {path:?}. Run `just build-dotnet-plugins` first.");
    path
}

fn phys_mb() -> f64 {
    memory_stats().map_or(0.0, |s| s.physical_mem as f64 / (1024.0 * 1024.0))
}

fn main() {
    let mem_start = phys_mb();
    println!("[1] baseline:            {mem_start:.2} MB");

    let rt = interoptopus_csharp::rt::dynamic::runtime().expect("Failed to create .NET runtime");
    let mem_after_runtime = phys_mb();
    println!("[2] after runtime init:  {mem_after_runtime:.2} MB  (+{:.2} MB)", mem_after_runtime - mem_start);

    let primitives = rt.load::<Primitives>(dll_path("functions_primitive.dll")).expect("Failed to load Primitives plugin");
    let mem_after_load = phys_mb();
    println!("[3] after plugin load:   {mem_after_load:.2} MB  (+{:.2} MB)", mem_after_load - mem_after_runtime);

    let result = primitives.primitive_u32(42);
    assert_eq!(result, 43);
    let mem_after_call = phys_mb();
    println!("[4] after fn call:       {mem_after_call:.2} MB  (+{:.2} MB)", mem_after_call - mem_after_load);

    drop(primitives);
    drop(rt);
    let mem_after_drop = phys_mb();
    println!("[5] after drop:          {mem_after_drop:.2} MB  (+{:.2} MB)", mem_after_drop - mem_after_call);

    println!();
    println!("total delta: {:.2} MB", mem_after_drop - mem_start);
}
