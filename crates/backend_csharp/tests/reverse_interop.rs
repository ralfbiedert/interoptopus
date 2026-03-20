#![allow(unused)]

use interoptopus::lang::plugin::PluginInfo;
use interoptopus_csharp::plugins::runtime::DotNetRuntime;
use reference_project::types::basic::Vec3f32;
use std::path::PathBuf;

fn plugin_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/plugins/dotnet_plugin/bin/Debug/net9.0")
}

fn dll_path() -> PathBuf {
    let path = plugin_dir().join("Plugin.dll");
    assert!(path.exists(), "Plugin DLL not found at {path:?}. Run `dotnet build` in tests/plugins/dotnet_plugin/ first.");
    path
}

interoptopus::plugin!(Plugin {
    fn do_math(x: i64, y: i64) -> i64;
    fn sum_all(x: i64, y: i64, z: u32) -> Vec3f32;

    impl Foo {
        fn create() -> Self;
        fn bar(&self, x: i32);
        // fn call(&self, x: &u32, cb: CallBack);
        // async fn call_async(&self, x: Wire<String>);
        fn get_accumulator(&self) -> i32;
    }
});

#[test]
fn can_load_and_call_do_math() {
    let runtime = DotNetRuntime::new().expect("Failed to create .NET runtime");
    let loader = runtime
        .dll_loader_with_namespace(dll_path().to_str().unwrap(), "My.Company")
        .expect("Failed to load DLL");

    let plugin = Plugin::new(&loader).expect("Failed to load MathPlugin");

    let vec = plugin.sum_all(123, 456, 789);
    dbg!(vec);

    let result = plugin.do_math(123, 456);
    assert_eq!(result, 579);
}

#[test]
fn can_load_foo_instance() {
    let runtime = DotNetRuntime::new().expect("Failed to create .NET runtime");
    let loader = runtime
        .dll_loader_with_namespace(dll_path().to_str().unwrap(), "My.Company")
        .expect("Failed to load DLL");

    let plugin = Plugin::new(&loader).expect("Failed to load MathPlugin");
    let foo = plugin.foo_create();

    assert_eq!(foo.get_accumulator(), 0);
    foo.bar(10);
    foo.bar(32);
    assert_eq!(foo.get_accumulator(), 42);

    drop(foo);
}

#[test]
fn can_run_dotnet_pipeline() {
    use interoptopus_csharp::DotnetLibrary;

    let output = DotnetLibrary::builder(Plugin::inventory()).build().process().expect("pipeline failed");

    output.write_buffers_to("tests/plugins/dotnet_plugin").expect("write failed");
}
