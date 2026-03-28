+++
title = "Installation"
weight = 2
disable_toc = true
+++

# Installation

## Requirements

- Rust 1.70 or later
- Cargo

## Adding to Your Project

Add the core crate to your `Cargo.toml`:

```rust
let x = 123;
```

Then add the backend crate for your target language, for example C#:

```toml
[build-dependencies]
interoptopus_backend_csharp = "0.4"
```

## Crate Types

Your library must be compiled as a `cdylib` to produce a native shared library:

```toml
[lib]
crate-type = ["cdylib"]
```
