use interoptopus::extra_type;
use interoptopus::inventory::{PluginInventory, RustInventory};
use interoptopus_proc::ffi;

#[allow(clippy::used_underscore_binding)]
#[ffi]
struct Foo {
    _x: u8,
}

#[test]
fn rust_inventory() {
    let _ = RustInventory::new().register(extra_type!(Foo)).validate();
}

#[test]
fn foreign_inventory() {
    let _ = PluginInventory::new().register(extra_type!(Foo)).validate();
}
