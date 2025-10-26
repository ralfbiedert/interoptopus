use backend_csharp_ng::dispatch::Dispatch;
use backend_csharp_ng::stage::output_master;
use backend_csharp_ng::{RustLibrary, RustLibraryConfig};
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
