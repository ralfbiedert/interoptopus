use interoptopus::{ffi_function, ffi_type, function, Inventory, InventoryBuilder};

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

// Helper to produce an `Inventory`, describing what our FFI library contains.
#[rustfmt::skip]
#[allow(unused)]
fn my_inventory() -> Inventory {
    InventoryBuilder::new()
        .register(function!(my_function))
        .validate()
        .inventory()
}

// We just trick a unit test into producing our bindings, here for C#
#[test]
fn generate_bindings() {
    use interoptopus::Interop;
    use interoptopus_backend_csharp::ConfigBuilder;
    use interoptopus_backend_csharp::Generator;

    let inventory = my_inventory();
    let config = ConfigBuilder::default().build().unwrap();

    Generator::new(config, inventory).write_file("bindings/Interop.cs").unwrap();
}
