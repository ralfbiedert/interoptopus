#![allow(unused)]

use interoptopus::lang::plugin::PluginInfo;
use interoptopus_csharp::plugins::runtime::DotNetRuntime;
use std::path::PathBuf;

fn plugin_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/plugins/dotnet_plugin/bin/Debug/net9.0")
}

fn dll_path() -> PathBuf {
    let path = plugin_dir().join("Plugin.dll");
    assert!(path.exists(), "Plugin DLL not found at {path:?}. Run `dotnet build` in tests/plugins/dotnet_plugin/ first.");
    path
}

interoptopus::plugin!(MathPlugin {
    fn do_math(x: i64, y: i64) -> i64;
});

interoptopus::plugin!(Foo {
    fn create() -> Self;
    fn bar(&self, x: i32);
    fn get_accumulator(&self) -> i32;
});

#[test]
fn can_load_and_call_do_math() {
    let runtime = DotNetRuntime::new().expect("Failed to create .NET runtime");
    let loader = runtime.dll_loader(dll_path().to_str().unwrap()).expect("Failed to load DLL");

    let plugin = MathPlugin::new(&loader).expect("Failed to load MathPlugin");

    let result = plugin.do_math(123, 456);
    assert_eq!(result, 579);
}

#[test]
fn can_load_foo_instance() {
    let runtime = DotNetRuntime::new().expect("Failed to create .NET runtime");
    let loader = runtime.dll_loader(dll_path().to_str().unwrap()).expect("Failed to load DLL");

    let foo = Foo::create(&loader).expect("Failed to create Foo");

    assert_eq!(foo.get_accumulator(), 0);
    foo.bar(10);
    foo.bar(32);
    assert_eq!(foo.get_accumulator(), 42);

    drop(foo);
}

#[test]
fn can_run_dotnet_pipeline() {
    use interoptopus::inventory::ForeignInventory;
    use interoptopus_csharp::DotnetLibrary;

    let mut inventory = ForeignInventory::new();
    MathPlugin::register(&mut inventory);
    Foo::register(&mut inventory);

    let output = DotnetLibrary::builder(inventory)
        .plugin_name("TestPlugin")
        .namespace("Test.Namespace")
        .build()
        .process()
        .expect("pipeline failed");

    output.write_buffers_to(".").expect("write failed");
}
