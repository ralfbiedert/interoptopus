// use interoptopus::ffi::CStrPtr;
use interoptopus::inventory::Inventory;
use interoptopus::lang::Wire;
use interoptopus::{ffi_function, ffi_type, function, pattern};

pub mod engine;
pub mod error;

#[ffi_type(wired)]
pub struct Something {
    field: u16,
    name: String,
}

#[ffi_type(wired)]
pub struct Return {
    field: u32,
}

// As in `engine`, we create matching functions that are better suited for an FFI boundary.
#[ffi_function(debug)]
pub fn start_server(mut server_name: Wire<Something>) -> Wire<Return> {
    // ^^ register should traverse Wire types recursively and add them to inventory
    // TypeInfo::wire_type_info() perhaps
    // NB: will c_types and wire_types ever mingle? --
    if server_name.name.is_empty() {
        return Return { field: 0 }; // @todo: This should generate Wire<>
    };

    core_library::start_server(server_name.name.to_string());
    Return { field: 1 }
}

pub fn ffi_inventory() -> Inventory {
    Inventory::builder()
        .register(function!(start_server))
        .register(pattern!(engine::GameEngine))
        .validate()
        .build()
}
