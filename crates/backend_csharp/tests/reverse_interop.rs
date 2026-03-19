#![allow(unused)]

use interoptopus::lang::plugin::Loader;
use interoptopus_csharp::plugins::runtime::DotNetRuntime;
use netcorehost::pdcstring::PdCString;
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

/// Hacky manual Foo handle to explore class instance loading via GCHandle.
struct FooHandle {
    handle: isize,
    bar_fn: unsafe extern "C" fn(isize, i32),
    get_accumulator_fn: unsafe extern "C" fn(isize) -> i32,
    drop_fn: unsafe extern "C" fn(isize),
}

impl FooHandle {
    fn bar(&self, x: i32) {
        unsafe { (self.bar_fn)(self.handle, x) }
    }

    fn get_accumulator(&self) -> i32 {
        unsafe { (self.get_accumulator_fn)(self.handle) }
    }
}

impl Drop for FooHandle {
    fn drop(&mut self) {
        unsafe { (self.drop_fn)(self.handle) }
    }
}

#[test]
fn can_load_foo_instance() {
    let dir = plugin_dir();
    let dll_path = dir.join("Plugin.dll");

    assert!(dll_path.exists(), "Plugin DLL not found at {dll_path:?}. Run `dotnet build` in tests/plugins/dotnet_plugin/ first.");

    let runtime = DotNetRuntime::new().expect("Failed to create .NET runtime");

    let dll_pdc = PdCString::from_os_str(dll_path.as_os_str()).unwrap();
    let delegate_loader = runtime.context().get_delegate_loader_for_assembly(dll_pdc).expect("Failed to get delegate loader");

    // Load all FooExports symbols
    let load_symbol = |method: &str| -> *const u8 {
        let type_pdc = PdCString::from_os_str("Plugin.Interop, Plugin".as_ref() as &std::ffi::OsStr).unwrap();
        let method_pdc = PdCString::from_os_str(method.as_ref() as &std::ffi::OsStr).unwrap();
        let managed_fn = delegate_loader
            .get_function_with_unmanaged_callers_only::<extern "system" fn()>(&type_pdc, &method_pdc)
            .unwrap_or_else(|e| panic!("Failed to load {method}: {e}"));
        let f: extern "system" fn() = *managed_fn;
        unsafe { std::mem::transmute::<extern "system" fn(), *const u8>(f) }
    };

    let new_fn: unsafe extern "C" fn() -> isize = unsafe { std::mem::transmute(load_symbol("foo_new")) };
    let bar_fn: unsafe extern "C" fn(isize, i32) = unsafe { std::mem::transmute(load_symbol("foo_bar")) };
    let get_acc_fn: unsafe extern "C" fn(isize) -> i32 = unsafe { std::mem::transmute(load_symbol("foo_get_accumulator")) };
    let drop_fn: unsafe extern "C" fn(isize) = unsafe { std::mem::transmute(load_symbol("foo_drop")) };

    // Create an instance
    let handle = unsafe { new_fn() };
    assert_ne!(handle, 0, "foo_new returned null");

    let foo = FooHandle {
        handle,
        bar_fn,
        get_accumulator_fn: get_acc_fn,
        drop_fn,
    };

    // Call methods
    assert_eq!(foo.get_accumulator(), 0);
    foo.bar(10);
    foo.bar(32);
    assert_eq!(foo.get_accumulator(), 42);

    // Drop frees the GCHandle
    drop(foo);
}
