use interoptopus::inventory::{Inventory, RustInventory};
use interoptopus::{callback, extra_type, ffi};

/// A simple type in our FFI layer.
#[ffi]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

/// A simple type in our FFI layer.
#[ffi]
pub enum Error {
    A,
    B(u32),
}

callback!(SumDelegate2(x: i32, y: i32) -> i32);
callback!(SumDelegateReturn(x: i32, y: i32) -> ffi::Result<(), Error>);
callback!(SumDelegateReturn2(x: i32, y: i32));
callback!(Pointers(x: &i32, y: &mut i32));
callback!(StringCallback(s: ffi::String));

/// Function using the type.
#[ffi]
pub fn my_function(input: Vec2) -> Vec2 {
    input
}

// We just trick a unit test into producing our bindings, here for C#
#[test]
#[rustfmt::skip]
fn generate_bindings() -> Result<(), Box<dyn std::error::Error>> {
    use interoptopus::function;
    use interoptopus::inventory::Inventory;
    use interoptopus_csharp::RustLibrary;

    // In a real project this should be a freestanding `my_inventory()` function inside
    // your FFI or build crate.
    let inventory = RustInventory::new()
        .register(function!(my_function))
        .register(extra_type!(Error))
        .register(extra_type!(SumDelegate2))
        .register(extra_type!(SumDelegateReturn))
        .validate();

    RustLibrary::builder(inventory)
        .dll_name("hello_world")
        .build()
        .process()?
        .write_buffers_to("bindings/")?;

    Ok(())
}
