use interoptopus::inventory::Inventory;

#[test]
fn basic() {
    let _ = Inventory::new().validate();
}
