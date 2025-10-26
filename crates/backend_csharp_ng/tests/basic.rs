use backend_csharp_ng::dispatch::Dispatch;
use backend_csharp_ng::stage::output_master;
use backend_csharp_ng::{RustLibrary, RustLibraryConfig};
use interoptopus::inventory::Inventory;

#[test]
fn rust_library() {
    let inventory = Inventory::new();
    let _ = RustLibrary::new(inventory).process().unwrap();
}

#[test]
fn rust_library_builder() {
    let inventory = Inventory::new();
    let _ = RustLibrary::builder(inventory).build();
}
