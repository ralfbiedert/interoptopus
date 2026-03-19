#![allow(unused)]

use interoptopus_csharp::plugins::runtime::DotNetRuntime;
use netcorehost::pdcstr;
use netcorehost::pdcstring::PdCString;
use std::path::PathBuf;

fn plugin_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/plugins/dotnet_plugin/bin/Debug/net9.0")
}

interoptopus::plugin!(MyPlugin {

    fn do_math(x: u32, y: u32);

    impl Foo {
        fn new() -> Self;
        fn bar(&self, x: u32);
    }
});

#[test]
fn can_load_and_call_add() {
    let dir = plugin_dir();
    let dll_path = dir.join("Plugin.dll");

    assert!(dll_path.exists(), "Plugin DLL not found at {dll_path:?}. Run `dotnet build` in tests/plugins/dotnet_plugin/ first.");

    let runtime = DotNetRuntime::new().expect("Failed to create .NET runtime");

    let dll_pdc = PdCString::from_os_str(dll_path.as_os_str()).unwrap();

    let loader = runtime.context().get_delegate_loader_for_assembly(dll_pdc).expect("Failed to get delegate loader");

    let add_fn1 = loader
        .get_function_with_unmanaged_callers_only::<fn()>(pdcstr!("Plugin.PluginExports, Plugin"), pdcstr!("Add"))
        .expect("Failed to load Add function");

    let add_fn = unsafe { std::mem::transmute::<_, fn(u32, u32) -> u32>(add_fn1) };

    let result = add_fn(123, 456);
    assert_eq!(result, 579);
}
