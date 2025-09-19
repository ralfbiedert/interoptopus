use interoptopus::extra_type;
use interoptopus::inventory::Inventory;

#[test]
fn basic() {
    let x = Inventory::new().register(extra_type!(u32)).validate();
}
