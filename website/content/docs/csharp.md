+++
title = "C# Backend"
weight = 10
+++

# C# Bindings

The C# backend generates idiomatic P/Invoke bindings for .NET.

## Setup

Add the backend to your build script dependencies:

```toml
[build-dependencies]
interoptopus_backend_csharp = "0.4"
```

## Generating Bindings

In your `build.rs`:

```rust
use interoptopus_backend_csharp::Interop;

fn main() {
    let inventory = my_crate::ffi_inventory();

    Interop::builder()
        .inventory(inventory)
        .namespace("MyProject")
        .build()
        .expect("failed to build interop")
        .write_file("bindings/Interop.cs")
        .expect("failed to write bindings");
}
```

## Features

- Structs map to C# `struct` or `class`
- Enums map to C# `enum` with optional payload support
- Services map to disposable C# classes
- `ffi::String`, `ffi::Vec<T>`, `ffi::Slice<T>` all have managed counterparts
- Async service constructors are supported
