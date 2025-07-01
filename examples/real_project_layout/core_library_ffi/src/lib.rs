use interoptopus::ffi::CStrPtr;
use interoptopus::inventory::Inventory;
use interoptopus::{ffi_function, function, pattern};

pub mod engine;
pub mod error;

// As in `engine`, we create matching functions that are better suited for an FFI boundary.
#[ffi_function]
pub fn start_server(server_name: Wire<Something>) -> Wire<Return> {
    // ^^ register should traverse Wire types recursively and add them to inventory
    // TypeInfo::wire_type_info() perhaps
    // will c_types and wire_types ever mingle?
    let Ok(name) = server_name.as_str() else {
        return;
    };

    core_library::start_server(name.to_string());
}

pub fn ffi_inventory() -> Inventory {
    Inventory::builder()
        .register(function!(start_server))
        .register(pattern!(engine::GameEngine))
        .validate()
        .build()
}
