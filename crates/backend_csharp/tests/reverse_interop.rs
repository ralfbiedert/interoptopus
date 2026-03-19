use interoptopus::plugin;
use netcorehost::pdcstr;
use netcorehost::pdcstring::PdCString;
use std::path::PathBuf;

fn plugin_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/plugins/dotnet_plugin/bin/Debug/net9.0")
}

plugin!(MyPlugin {

    fn do_math(x: u32, y: u32);

    impl Foo {
        fn new() -> Self;
        fn bar(&self, x: u32);
    }
});

struct MyPlugin {}

impl MyPlugin {
    fn do_math(&self, x: u32, y: u32) {
        todo!()
    }
}

struct A;
struct B;

struct Foo<T = B> {
    _t: std::marker::PhantomData<T>,
}

impl Foo<A> {
    pub fn from(my_plugin: &MyPlugin) -> Foo<B> {
        todo!()
    }
}

impl Foo<B> {
    pub fn bar(&self, x: u32) {}
}

#[test]
fn can_load_and_call_add() {
    let dir = plugin_dir();
    let dll_path = dir.join("Plugin.dll");
    let config_path = dir.join("Plugin.runtimeconfig.json");

    assert!(dll_path.exists(), "Plugin DLL not found at {dll_path:?}. Run `dotnet build` in tests/plugins/dotnet_plugin/ first.");

    let fxr = netcorehost::nethost::load_hostfxr().expect("Failed to load hostfxr");

    let config_pdc = PdCString::from_os_str(config_path.as_os_str()).unwrap();
    let dll_pdc = PdCString::from_os_str(dll_path.as_os_str()).unwrap();

    let context = fxr.initialize_for_runtime_config(config_pdc).expect("Failed to initialize runtime");

    let loader = context.get_delegate_loader_for_assembly(dll_pdc).expect("Failed to get delegate loader");

    let add_fn = loader
        .get_function_with_unmanaged_callers_only::<fn(i64, i64) -> i64>(pdcstr!("Plugin.PluginExports, Plugin"), pdcstr!("Add"))
        .expect("Failed to load Add function");

    let result = add_fn(123, 456);
    assert_eq!(result, 579);
}
