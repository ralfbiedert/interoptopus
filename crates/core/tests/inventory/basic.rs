use interoptopus::inventory::RustInventory;

#[test]
fn basic() {
    let _ = RustInventory::new().validate();
}
