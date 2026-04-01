+++
title = "Getting Started"
weight = 30
+++

This guide walks you through setting up a minimal Interoptopus project and generating your first C# bindings. For other use cases see the [Examples](@/examples/_index.md) section.

## 1. Add the dependency

In your library's `Cargo.toml`:

```toml
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
interoptopus = "..." # Pick the latest version

[dev-dependencies]
interoptopus_csharp = "..." # Pick the latest version
```

Both crate types are required: `cdylib` produces the native `.dll`/`.so` your C# project loads at runtime, and `rlib` allows tests (including the binding generator) to call back into the same crate.

## 2. Annotate your items

Inside your project:

```rust
use interoptopus::ffi;

#[ffi]
pub fn hello_world()  {}
```

`#[ffi]` on a function validates that all parameters are FFI-safe and makes it available for FFI use.

## 3. Build an inventory

```rust
use interoptopus::{function, inventory::RustInventory};

pub fn ffi_inventory() -> RustInventory {
    RustInventory::new()
        .register(function!(hello_world))
        .validate()
}
```

The inventory is the list of everything your FFI surface exports. 


## 4. Generate bindings

```rust
use interoptopus_csharp::RustLibrary;

#[test]
fn generate_bindings() -> Result<(), Box<dyn std::error::Error>> {
    RustLibrary::builder(ffi_inventory())
        .dll_name("my_library")
        .build()
        .process()?
        .write_buffers_to("bindings/")?;
    Ok(())
}
```

This writes your bindings into the specified folder.

