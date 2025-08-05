use anyhow::Error;
use interoptopus::backend::NamespaceMappings;
use interoptopus::{
    builtins_string, function,
    inventory::{Bindings, Inventory},
    lang::Type,
};
use interoptopus_backend_csharp::{Interop, WriteTypes};

// Test wire types with basic FFI-compatible types
use interoptopus::{
    ffi_function, ffi_type,
    lang::{Wire, Wireable},
};

#[ffi_type(wired, debug)]
pub struct SimpleSub {
    pub empty: bool,
}

#[ffi_type(wired, debug)]
pub struct SimpleInput {
    pub id: i32,
    pub enabled: bool,
    pub count: u64,
    pub sub: SimpleSub,
}

#[ffi_type(wired, debug)]
pub struct SimpleOutput {
    pub result: Vec<i32>,
    pub success: bool,
}

#[ffi_function(debug)]
fn process_simple_data(mut input: Wire<SimpleInput>) -> Wire<SimpleOutput> {
    let input_data = input.unwire().unwrap();
    let mut vec = Vec::with_capacity(input_data.count as usize);
    for _ in 0..input_data.count {
        vec.push(input_data.id);
    }
    let output = SimpleOutput { result: vec, success: input_data.enabled };
    output.wire()
}

fn main() -> Result<(), Error> {
    println!("Testing wire type field generation fix...");

    let inventory = Inventory::builder()
        .register(builtins_string!())
        .register(function!(process_simple_data))
        .validate()
        .build();

    println!("Inventory contains {} types", inventory.c_types().len());

    // Verify that wired types have fields
    let mut wired_types_with_fields = 0;
    let mut wired_types_with_fields_names = vec![];
    for t in inventory.c_types() {
        println!("{t:?}");
        if let Type::Wired(w) = t {
            println!("Wired type '{}' has {} fields:", w.rust_name(), w.fields().len());
            for field in w.fields() {
                println!("  - {}: {:?}", field.name(), field.the_type());
            }
            if !w.fields().is_empty() {
                wired_types_with_fields += 1;
                wired_types_with_fields_names.push(w.rust_name());
            }
        }
    }

    if wired_types_with_fields > 0 {
        println!("✅ SUCCESS: Found {} wired types with fields! {}", wired_types_with_fields, wired_types_with_fields_names.join(", "));
    } else {
        println!("❌ FAILED: No wired types have fields");
        return Err(anyhow::anyhow!("Field generation failed"));
    }

    // Generate C# output
    let interop = Interop::builder()
        .inventory(inventory)
        .namespace_mappings(NamespaceMappings::new("WireTest"))
        .dll_name("wire_test".to_string())
        .write_types(WriteTypes::NamespaceAndInteroptopusGlobal)
        .build()?;

    interop.write_file("./WireTest.cs")?;
    println!("Generated C# bindings to WireTest.cs");

    Ok(())
}
