use backend_csharp_ng::RustLibrary;
use interoptopus::inventory::RustInventory;

#[test]
fn rust_library() {
    let inventory = RustInventory::new();
    let _ = RustLibrary::new(inventory).process().unwrap();
}

#[test]
fn rust_library_builder() {
    let inventory = RustInventory::new();
    let _ = RustLibrary::builder(inventory).build();
}

#[test]
fn real_inventory() {
    let reference_project = include_str!("inventory/reference_project.json");
    let inventory: RustInventory = serde_json::from_str::<RustInventory>(reference_project).unwrap();
    let library = RustLibrary::new(inventory);
    let result = library.process();

    assert!(result.is_ok())
}

#[test]
fn real_inventory_temp() {
    let reference_project = include_str!("inventory/reference_project.json");
    let inventory: RustInventory = serde_json::from_str::<RustInventory>(reference_project).unwrap();
    let library = RustLibrary::new(inventory);
    let multibuf = library.process().unwrap();
    let result = multibuf.write_buffer("Foo.cs");

    // result
}
