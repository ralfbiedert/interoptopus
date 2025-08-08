//!
//! Interoptopus FFI-types based ipc
//!
use interoptopus::{builtins_string, builtins_vec, function, inventory::Inventory};

pub mod ffi;
pub mod wire;

pub use ffi::FfiRustClient;
pub use wire::WireRustClient;

pub fn ffi_inventory() -> Inventory {
    let inventory = Inventory::builder()
        .register(builtins_string!())
        .register(builtins_vec!(u8))
        .register(builtins_vec!(interoptopus::ffi::String))
        .register(function!(FfiRustClient))
        .register(function!(WireRustClient))
        .register(builtins_vec!(ffi::Item))
        .register(builtins_vec!(ffi::Result))
        .validate()
        .build();

    inventory.debug();

    inventory
}
