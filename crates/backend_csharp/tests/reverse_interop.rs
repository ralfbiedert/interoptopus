#![allow(unused)]

use interoptopus::ffi;
use interoptopus::lang::plugin::Loader;
use interoptopus_csharp::plugins::runtime::DotNetRuntime;
use std::collections::HashMap;
use std::path::PathBuf;

fn plugin_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/plugins/dotnet_plugin/bin/Debug/net9.0")
}

#[ffi]
struct Data {
    context: HashMap<String, String>,
}

interoptopus::plugin!(MyPlugin {
    fn do_math(x: i64, y: i64) -> i64;

    impl Foo {
        fn create() -> Self;
        fn bar(&self, x: i32);
        fn get_accumulator(&self) -> i32;
    }
});

#[test]
fn can_load_and_call_do_math() {
    let dir = plugin_dir();
    let dll_path = dir.join("Plugin.dll");

    assert!(dll_path.exists(), "Plugin DLL not found at {dll_path:?}. Run `dotnet build` in tests/plugins/dotnet_plugin/ first.");

    let runtime = DotNetRuntime::new().expect("Failed to create .NET runtime");
    let plugin: MyPlugin = runtime.load_plugin(dll_path.to_str().unwrap()).expect("Failed to load plugin");

    let result = plugin.do_math(123, 456);
    assert_eq!(result, 579);
}

#[test]
fn can_load_foo_instance() {
    let dir = plugin_dir();
    let dll_path = dir.join("Plugin.dll");

    assert!(dll_path.exists(), "Plugin DLL not found at {dll_path:?}. Run `dotnet build` in tests/plugins/dotnet_plugin/ first.");

    let runtime = DotNetRuntime::new().expect("Failed to create .NET runtime");
    let plugin: MyPlugin = runtime.load_plugin(dll_path.to_str().unwrap()).expect("Failed to load plugin");

    let foo = plugin.foo_create();

    // Call methods
    assert_eq!(foo.get_accumulator(), 0);
    foo.bar(10);
    foo.bar(32);
    assert_eq!(foo.get_accumulator(), 42);

    // Drop frees the GCHandle
    drop(foo);
}
