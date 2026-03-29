+++
title = "Hello World"
weight = 100
+++

The [hello_world](https://github.com/ralfbiedert/interoptopus/tree/master/examples/hello_world) showcases the smallest possible Interoptopus project, exporting a single function to C# in the most straightforward way.


## Project Structure

```
hello_world/
├── Cargo.toml
├── src/
│   └── lib.rs        ← FFI types, functions, and binding generation
└── bindings/
    └── Interop.cs    ← Generated C# output
```

Everything lives in one crate. This works well for quick experiments or simple libraries.

## The Rust Side

```rust
use interoptopus::ffi;

#[ffi]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[ffi]
pub fn my_function(input: Vec2) -> Vec2 {
    input
}
```

The `#[ffi]` attribute on a struct makes it FFI-safe, and ensures it actually is. Using `#[ffi]` on a function marks it for export and makes it available for binding generation (and again, validates its parameters are safe). 

## Generating Bindings

This example uses a unit test to trigger binding generation:

```rust
#[test]
fn generate_bindings() -> Result<(), Box<dyn std::error::Error>> {
    let inventory = RustInventory::new()
        .register(function!(super::my_function))
        .validate();

    RustLibrary::builder(inventory)
        .dll_name("hello_world")
        .build()
        .process()?
        .write_buffers_to("bindings/")?;

    Ok(())
}
```

It consists of two parts:

- **Inventory generation** — A `RustInventory` collects everything that should cross the FFI boundary. You register each function explicitly, types are inferred. The inventory is the single source of truth the backend uses to know what to generate.
- **Binding emission** — The C# backend emitter `RustLibrary` takes the inventory and drives the interop generation. Notably, you also need `.dll_name("hello_world")` to tell it what native library to load, and `.process()` runs all included codegen passes. The final `.write_buffers_to("bindings/")` writes the resulting `.cs` files to disk.


## The Cargo.toml

```toml
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
interoptopus = { ..., features = ["macros"] }

[dev-dependencies]
interoptopus_csharp = ...
```

Most importantly, remember to set `crate-type`. The type `cdylib` produces the native `.dll`/`.so` that your C# app loads at runtime, while `rlib` allows the unit test to call back into the same crate to read type information for binding generation.

## The Generated C# Side

`Interop.cs` then contains an `Interop` class with a `[LibraryImport]` declaration for each exported function, plus a `struct Vec2` with its managed and unmanaged representations and the marshalling glue between them, which you can use like so:

```csharp
var vec2 = Interop.my_function(new Vec2 {});
```


In a real-world project you probably don't want to do it like depicted here (see the [production project](@/examples/production_project/_index.md)), which keeps your core logic cleanly separated from FFI concerns.
