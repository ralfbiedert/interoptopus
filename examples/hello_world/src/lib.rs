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
    use interoptopus_csharp::RustLibrary;
    use interoptopus_csharp::dispatch::Dispatch;
    use interoptopus_csharp::lang::meta::FileEmission;
    use interoptopus_csharp::output::Target;

    // We just trick a unit test into producing our bindings, here for C#
    #[test]
    fn generate_bindings() -> Result<(), Box<dyn std::error::Error>> {
        // In a real project this should be a freestanding `my_inventory()` function inside
        // your FFI or build crate.
        let inventory = RustInventory::new().register(function!(super::my_function)).validate();

        RustLibrary::builder(inventory)
            .dll_name("hello_world")
            .dispatch(Dispatch::custom(|x, _| match x.emission {
                FileEmission::Common => Target::new("Interop.Common.cs", "My.Company.Common"),
                FileEmission::Default => Target::new("Interop.cs", "My.Company"),
                FileEmission::CustomModule(_) => Target::new("Interop.cs", "My.Company"),
            }))
            .build()
            .process()?
            .write_buffers_to("bindings/")?;

        Ok(())
    }
}
