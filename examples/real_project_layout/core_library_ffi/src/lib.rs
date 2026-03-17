// use interoptopus::ffi::CStrPtr;
use interoptopus::inventory::Inventory;
use interoptopus::lang::types::WireIO;
use interoptopus::wire::Wire;
use interoptopus::{ffi, function, pattern};

pub mod engine;
pub mod error;

#[ffi]
pub struct Something {
    field: u16,
    name: String,
}

#[ffi]
pub struct Return {
    field: u32,
}

// As in `engine`, we create matching functions that are better suited for an FFI boundary.
#[ffi]
pub fn start_server(mut server_name: Wire<Something>) -> Wire<Return> {
    let server_name = server_name.unwire().unwrap();
    let result = if server_name.name.is_empty() {
        Return { field: 0 }
    } else {
        core_library::start_server(server_name.name.to_string());
        Return { field: 1 }
    };
    let mut out = Wire::with_size(result.live_size());
    out.serialize(&result).unwrap();
    out
}

pub fn ffi_inventory() -> Inventory {
    Inventory::builder()
        .register(function!(start_server))
        .register(pattern!(engine::GameEngine))
        .validate()
        .build()
}
