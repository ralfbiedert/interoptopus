use interoptopus::{builtins_string, builtins_vec, builtins_wire, function, inventory::Inventory};

pub fn ffi_inventory() -> Inventory {
    let inventory = Inventory::builder()
        .register(builtins_string!())
        .register(builtins_wire!())
        .register(builtins_vec!(u8))
        .register(builtins_vec!(interoptopus::ffi::String))
        .register(function!(crate::ffi::FfiRustClient))
        .register(function!(crate::wire::WireRustClient))
        .register(builtins_vec!(crate::ffi::FItem))
        .register(builtins_vec!(crate::ffi::FResult))
        .validate()
        .build();

    inventory.debug();

    inventory
}
