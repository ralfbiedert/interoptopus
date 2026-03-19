#![allow(unused)]

use interoptopus::lang::plugin::Loader;
use interoptopus_csharp::plugins::runtime::DotNetRuntime;
use std::path::PathBuf;

fn plugin_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/plugins/dotnet_plugin/bin/Debug/net9.0")
}

interoptopus::plugin!(MyPlugin {
    fn do_math(x: i64, y: i64) -> i64;

    impl Foo {
        fn new() -> Self;
        fn bar(&self, x: u32);
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
