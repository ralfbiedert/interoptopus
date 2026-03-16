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
