use netcorehost::pdcstr;
use std::thread::sleep;
use std::time::Duration;

trait ILogger {
    fn log(&self, msg: &str);
}

struct MyLogger;

impl ILogger for MyLogger {}

ffi_plugin!(MyPlugin {
    // TODO: how to do ILogger that wants to be passed in as libs require it.

    fn do_math(x: u32, y: u32);

    trait Bar {
        fn bar(ILogger: &ILogger);
    }
});

#[test]
fn can_load() {
    sleep(Duration::from_secs(5));

    let my_logger = MyLogger;

    let runtime = DotNet::new();
    let plugin = runtime.load_dll::<MyPlugin>("tests/runtime/bin/Debug/net9.0/plugin.dll")?;
    plugin.do_math(123, 456);

    let fxr = netcorehost::nethost::load_hostfxr().unwrap();
    let r = fxr
        .initialize_for_runtime_config(pdcstr!("tests/runtime/bin/Debug/net9.0/plugin.runtimeconfig.json"))
        .unwrap();

    let loader = r.get_delegate_loader_for_assembly(pdcstr!("tests/runtime/bin/Debug/net9.0/plugin.dll")).unwrap();
    let myfn = loader
        .get_function_with_unmanaged_callers_only::<fn(u64, u64) -> u64>(pdcstr!("Plugin.PluginExports, Plugin"), pdcstr!("DoMath"))
        .unwrap();

    let x = myfn(123, 456);
    sleep(Duration::from_secs(5));
    dbg!(579);
}
