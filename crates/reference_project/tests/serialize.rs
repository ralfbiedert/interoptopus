use interoptopus_reference_project::ffi_inventory;
use std::fs;

#[test]
#[ignore = "For debugging, writes inventory to a file."]
pub fn serialize_rust_inventory() {
    let inventory = ffi_inventory();
    let result = serde_json::to_string_pretty(&inventory).unwrap();
    fs::write("foo.json", result).unwrap();
}
