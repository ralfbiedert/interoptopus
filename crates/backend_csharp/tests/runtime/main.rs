use netcorehost::pdcstr;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn can_load() {
    sleep(Duration::from_secs(5));

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
