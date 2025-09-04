use anyhow::Error;
use interoptopus::inventory::Inventory;
use interoptopus::{callback, ffi, ffi_function, ffi_type, function};
use interoptopus_backend_csharp::Interop;
use tests::backend_csharp::common_namespace_mappings;

#[ffi_type(debug)]
pub struct KeyValuePair<'a> {
    key: ffi::Slice<'a, u8>,
    value: ffi::Slice<'a, u8>,
}

callback!(LoggerCallback(fields: ffi::Vec<KeyValuePair<'_>>));

#[ffi_function(debug)]
pub extern "C" fn init_logger(_callback: LoggerCallback) {}

fn ffi_inventory() -> Inventory {
    Inventory::builder().register(function!(init_logger)).validate().build()
}

#[test]
fn bug_reproduction() -> Result<(), Error> {
    let generated = Interop::builder()
        .inventory(ffi_inventory())
        .namespace_mappings(common_namespace_mappings())
        .build()?
        .to_string()?;

    println!("Generated C# code:");
    println!("{}", generated);

    // Also write to file for easier inspection
    std::fs::write("bug_reproduction_output.cs", &generated).unwrap();

    // Check if we have the bug - looking for "Slicebyte" instead of "SliceU8"
    // Let's look specifically at the struct field definition lines
    let struct_lines: Vec<&str> = generated
        .lines()
        .filter(|line| line.trim_start().starts_with("Slice") && line.contains("key") || line.contains("value"))
        .collect();

    println!("Struct field lines: {:?}", struct_lines);

    if generated.contains("Slicebyte") {
        panic!("Found bug: 'Slicebyte' instead of 'SliceU8' in generated code");
    }

    Ok(())
}
