# interoptopus_csharp

C# backend for [Interoptopus](https://crates.io/crates/interoptopus).

Generates idiomatic C# bindings from a Rust FFI library, including classes for services, delegates for callbacks,
and most Interoptopus patterns.

## Usage

Add the crate as a dependency:

```toml
[dependencies]
interoptopus_csharp = "..."
```

Then write a test that builds an inventory and runs the backend:

```rust
use interoptopus::inventory::RustInventory;
use interoptopus::{ffi, function};
use interoptopus_csharp::RustLibrary;

#[ffi]
pub fn my_function(x: u32) -> u32 { x + 1 }

#[test]
fn generate_bindings() -> Result<(), Box<dyn std::error::Error>> {
    let inventory = RustInventory::new()
        .register(function!(my_function))
        .validate();

    RustLibrary::builder(inventory)
        .dll_name("my_lib")
        .build()
        .process()?
        .write_buffers_to("bindings/")?;

    Ok(())
}
```

This produces an `Interop.cs` file in `bindings/` with `[DllImport("my_lib")]` declarations
and idiomatic C# wrappers for all registered items.

For multi-file output or custom namespaces, use a [`Dispatch`](https://docs.rs/interoptopus_csharp/latest/interoptopus_csharp/dispatch/struct.Dispatch.html):

```rust
use interoptopus_csharp::dispatch::Dispatch;
use interoptopus_csharp::lang::meta::FileEmission;
use interoptopus_csharp::output::Target;

let dispatch = Dispatch::custom(|item, _| match item.emission {
    FileEmission::Common => Target::new("Interop.Common.cs", "My.Company.Common"),
    FileEmission::Default => Target::new("Interop.cs", "My.Company"),
    FileEmission::CustomModule(_) => Target::new("Interop.cs", "My.Company"),
});
```


