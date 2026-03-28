+++
title = "Introduction"
weight = 1
+++

**Interoptopus** is a polyglot FFI bindings generator for Rust libraries. Write your Rust library once and generate idiomatic bindings for multiple target languages.

## Supported Languages

| Language | Crate |
|----------|-------|
| C#       | `interoptopus_backend_csharp` |
| C        | `interoptopus_backend_c` |
| Python   | `interoptopus_backend_cpython` |

## Quick Start

Add interoptopus to your `Cargo.toml`:

```toml
[dependencies]
interoptopus = "0.4"

[lib]
crate-type = ["cdylib"]
```

Annotate your FFI surface with `#[ffi]`:

```rust
use interoptopus::ffi;

#[ffi]
pub struct MyStruct {
    pub value: u32,
}

#[ffi]
pub fn my_function(input: MyStruct) -> u32 {
    input.value * 2
}
```

Then build an inventory and generate bindings:

```rust
pub fn ffi_inventory() -> interoptopus::RustInventory {
    interoptopus::RustInventory::new()
        .register(interoptopus::function!(my_function))
        .validate()
}
```
