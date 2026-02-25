use interoptopus::ffi;

/// A simple type in our FFI layer.
#[ffi]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

/// Function using the type.
#[ffi]
pub fn my_function(input: Vec2) -> Vec2 {
    input
}

#[cfg(test)]
mod tests {
    use interoptopus::function;
    use interoptopus::inventory::RustInventory;

    fn inventory() -> RustInventory {
        RustInventory::new().register(function!(super::my_function)).validate()
    }

    // We just trick unit tests into producing our bindings.

    #[test]
    fn generate_csharp_bindings() -> Result<(), Box<dyn std::error::Error>> {
        use interoptopus_csharp::RustLibrary;

        RustLibrary::builder(inventory())
            .dll_name("hello_world")
            .build()
            .process()?
            .write_buffers_to("bindings_csharp/")?;

        Ok(())
    }

    #[test]
    fn generate_c_bindings() -> Result<(), Box<dyn std::error::Error>> {
        interoptopus_c::generate("hello_world", &inventory(), "bindings_c/hello_world.h")?;
        Ok(())
    }
}
