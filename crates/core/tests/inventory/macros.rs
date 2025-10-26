use interoptopus::extra_type;
use interoptopus::inventory::{ForeignInventory, RustInventory};
use interoptopus_proc::ffi;

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
    let _ = ForeignInventory::new().register(extra_type!(Foo)).validate();
}
