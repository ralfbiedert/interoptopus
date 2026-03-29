use interoptopus::ffi;
use interoptopus::inventory::RustInventory;
use interoptopus::{function, service};

pub mod engine;
pub mod error;

/// Starts the game server. Returns 1 on success, 0 if the name is empty.
#[ffi]
pub fn start_server(server_name: ffi::CStrPtr) -> u32 {
    let name = server_name.as_str().unwrap_or("");
    if name.is_empty() {
        0
    } else {
        core_library::start_server(name.to_string());
        1
    }
}

pub fn ffi_inventory() -> RustInventory {
    RustInventory::new()
        .register(function!(start_server))
        .register(service!(engine::GameEngine))
        .validate()
}
