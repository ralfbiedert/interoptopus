use interoptopus::inventory::RustInventory;
use interoptopus_csharp::RustLibrary;

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
fn real_inventory() -> Result<(), Box<dyn std::error::Error>> {
    let reference_project = include_str!("inventory/reference_project.json");
    let inventory: RustInventory = serde_json::from_str::<RustInventory>(reference_project)?;
    let library = RustLibrary::new(inventory);
    let result = library.process();

    assert!(result.is_ok());

    Ok(())
}

#[test]
fn real_inventory_temp() -> Result<(), Box<dyn std::error::Error>> {
    let reference_project = include_str!("inventory/reference_project.json");
    let inventory: RustInventory = serde_json::from_str::<RustInventory>(reference_project)?;
    let library = RustLibrary::builder(inventory).dll_name("foo").build();
    let multibuf = library.process()?;
    multibuf.write_buffer("Interop.cs")?;

    Ok(())
}
