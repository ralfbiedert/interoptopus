+++
title = "Production Project"
weight = 200
+++

The [production_project](https://github.com/ralfbiedert/interoptopus/tree/master/examples/production_project) example shows how to structure a real Interoptopus project across multiple crates. 

It models a scenario most teams will encounter: an existing Rust library that you want to expose to other languages without contaminating the core code with FFI concerns.

## Project Structure

```
production_project/
├── core_library/           ← Your pure Rust library (no FFI)
├── core_library_ffi/       ← FFI glue crate
└── core_library_ffi_build/ ← cdylib + build.rs for binding generation
```

Here we have three crates, each with a separate responsibility.

## Pure Rust Core

Your `core_library` crate knows nothing about Interoptopus. 

```rust
pub struct GameEngine { pub object_count: u32 }

impl GameEngine {
    pub fn place_object(&mut self, name: &str, position: Vec2) { ... }
    pub fn num_objects(&self) -> u32 { ... }
}
```

It uses idiomatic Rust types (`&str`, plain structs) and has no `#[ffi]` attributes. You can write tests for it, publish it to crates.io, and iterate on it without ever thinking about foreign languages.

## FFI Glue

The `core_library_ffi` crate depends on `core_library` and defines an interop layer. 


```rust
#[ffi(service)]
pub struct GameEngine {
    engine: core_library::engine::GameEngine,  // contains the native type
}

#[ffi]
impl GameEngine {
    pub fn create() -> ffi::Result<Self, Error> { ... }
    pub fn place_object(&mut self, name: ffi::CStrPtr, position: Vec2) -> ffi::Result<(), Error>
    pub fn num_objects(&self) -> u32 { ... }
}

#[ffi]
pub struct Vec2 { pub x: f32, pub y: f32 }
```

We define this FFI analog because your `core_library` types can have (in the real world) semantics that are ugly or impossible to express over FFI. 

While in many cases you could just add `#[ffi]` annotations to your core items, experience has shown that this often leads to painful refactorings down the line once the semantics can't be papered over anymore. 
The purpose of Interoptopus here isn't to avoid defining that adaptation layer —  we consider it a best practice after all — but to make its creation as painless and efficient as possible. 


The crate then also owns the inventory function:

```rust
pub fn ffi_inventory() -> RustInventory {
    RustInventory::new()
        .register(function!(start_server))
        .register(service!(engine::GameEngine))
        .validate()
}
```


## The Build Crate


The `core_library_ffi_build`  crate has two roles, driven by its `Cargo.toml`:

```toml
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
core_library_ffi = { path = "../core_library_ffi" }

[build-dependencies]
core_library_ffi = { path = "../core_library_ffi" }
interoptopus_csharp = ...
```

For one, it compiles `core_library_ffi` into the native `.dll`/`.so` your C# app loads. The `lib.rs` is a one-liner that just re-exports everything again (working around a Cargo bug):

```rust
pub use core_library_ffi::*;
```

Also, via `build.rs`, it generates C# bindings automatically whenever you run `cargo build`:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    RustLibrary::builder(ffi_inventory())
        .dll_name("core_library")
        .build()
        .process()?
        .write_buffers_to("bindings/")?;
    Ok(())
}
```

The key advantage over the [Hello World](@/examples/hello_world/_index.md) unit-test approach is that `cargo build` produces both the `.dll` and the `.cs` files in a single step. 


