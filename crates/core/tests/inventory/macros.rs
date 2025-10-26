use interoptopus::function;
use interoptopus::inventory::{ForeignInventory, RustInventory};
use interoptopus_proc::ffi;

#[ffi]
fn foo() {}

#[test]
fn rust_inventory() {
    let _ = RustInventory::new().register(function!(foo)).validate();
}

#[test]
fn foreign_inventory() {
    let _ = ForeignInventory::new().register(function!(foo)).validate();
}
