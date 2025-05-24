use interoptopus::{ffi_function, ffi_type};

/// A simple type in our FFI layer.
#[ffi_type]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

/// Function using the type.
#[ffi_function]
pub fn my_function(input: Vec2) -> Vec2 {
    input
}

// We just trick a unit test into producing our bindings, here for C#
#[test]
#[rustfmt::skip]
fn generate_bindings() {
    use interoptopus::function;
    use interoptopus_backend_csharp::InteropBuilder;
    use interoptopus::inventory::{Bindings, Inventory};

    // In a real project this should be a freestanding `my_inventory()` function inside
    // your FFI or build crate.
    let inventory = Inventory::builder()
        .register(function!(my_function))
        .validate()
        .build();

    InteropBuilder::new()
        .inventory(inventory)
        .build().unwrap()
        .write_file("bindings/Interop.cs").unwrap()
}
