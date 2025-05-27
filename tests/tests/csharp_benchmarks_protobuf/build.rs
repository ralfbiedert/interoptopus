//================
//=== PROTOBUF ===
//================

fn compile_proto_files() {
    let proto_files = ["Input.proto", "Outputs.proto"];
    let include_paths = ["/.", "./models"];

    for file in &proto_files {
        println!("cargo:rerun-if-changed={file}");
    }

    prost_build::compile_protos(&proto_files, &include_paths).unwrap();
}

//====================
//=== INTEROPTOPUS ===
//====================
use anyhow::Error;
use interoptopus::{
    function,
    inventory::{Bindings, Inventory, InventoryBuilder},
    builtins_string, builtins_vec
};
use interoptopus_backend_csharp::{InteropBuilder, WriteTypes};

fn ffi_inventory() -> Inventory {
    InventoryBuilder::new()
        .register(builtins_string!())
        .register(builtins_vec!(u8))
        .register(function!(crate::ffi::FfiRustClient))
        .validate()
        .build()
}

// use tests::backend_csharp::common_namespace_mappings;
use interoptopus::backend::NamespaceMappings;
pub fn namespace_mappings() -> NamespaceMappings {
    NamespaceMappings::new("ForCSharp.Ffi").add("common", "ForCSharp.Common")
    //.add("wire", "ForCSharp.Wire")
}

// use tests::validate_output;
#[macro_export]
macro_rules! validate_output {
    ($folder:expr, $file:expr, $generated:expr) => {
        let file = format!("{}/{}", $folder, $file);

        if true {
            std::fs::write(file, $generated).unwrap();
        } else {
            let expected = std::fs::read_to_string(file)?;
            assert_eq!($generated, expected);
        }
    };
}

fn generate_interopt_files() -> Result<(), Error>
{
    let generated_common = InteropBuilder::new()
        .inventory(ffi_inventory())
        .namespace_id("common".to_string())
        .namespace_mappings(namespace_mappings())
        .dll_name("proto_benchy".to_string())
        .write_types(WriteTypes::NamespaceAndInteroptopusGlobal)
        .build()?
        .to_string()?;

/*    let generated_wire = InteropBuilder::new()
        .inventory(ffi_inventory()) // FIXME: wire types are not part of inventory
        .namespace_id("wire".to_string())
        .namespace_mappings(namespace_mappings())
        .dll_name("proto_benchy".to_string())
        .write_types(WriteTypes::Namespace)
        .build()?
        .to_string()?;*/

    let generated_ffi = InteropBuilder::new()
        .inventory(ffi_inventory())
        .namespace_mappings(namespace_mappings())
        .dll_name("proto_benchy".to_string())
        .write_types(WriteTypes::Namespace)
        .build()?
        .to_string()?;

    validate_output!("./", "Interop.Common.cs", generated_common.as_str());
//    validate_output!("./", "Interop.Wire.cs", generated_wire.as_str());
    validate_output!("./", "Interop.Ffi.cs", generated_ffi.as_str());

    Ok(())
}

fn main() {
    compile_proto_files();
    generate_interopt_files().unwrap();
}
