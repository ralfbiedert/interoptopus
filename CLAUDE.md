# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Interoptopus is a polyglot bindings generator for Rust libraries. It allows you to write a Rust library once and generate idiomatic bindings for multiple target languages (C#, C, Python). The core concept is using FFI-safe Rust code with special attributes (`#[ffi_type]`, `#[ffi_function]`) that get processed into language-specific bindings.

## Development Commands

### Core Commands
- `cargo build` - Build all workspace crates
- `cargo test` - Run all tests (also generates bindings when `INTEROPTOPUS_UPDATE_BINDINGS=1`)
- `just update-bindings` - Update generated bindings with env var set

### Testing with Binding Updates
```bash
INTEROPTOPUS_UPDATE_BINDINGS=1 cargo test
```
This runs tests and regenerates bindings for all target languages.

### Working with Individual Crates
- `cargo build -p interoptopus` - Build core crate
- `cargo test -p interoptopus_backend_csharp` - Test C# backend
- `cargo check --workspace` - Quick check all crates

## Architecture

### Workspace Structure
```
crates/
├── core/                   # Main interoptopus crate with core FFI abstractions
├── proc_macros/            # Proc macro crate used by core
├── proc_macros_impl/       # Procedural macros (#[ffi_type], #[ffi_function]) incl. testing function
├── backend_c/              # C header generation
├── backend_csharp/         # C# bindings generation
├── backend_cpython/        # Python bindings generation
├── backend_utils/          # Shared utilities for backends
└── reference_project/      # Comprehensive test project using all features
```

### Core Components

**Inventory System**: The central registry that tracks all FFI types, functions, constants, and patterns. Libraries create an `Inventory` to register their FFI surface.

**FFI Types**: Rust types marked with `#[ffi_type]` can cross FFI boundaries safely. Includes primitives, structs, enums, and special patterns.

**FFI Functions**: Rust functions marked with `#[ffi_function]` can be exported for FFI use with when registered with an inventory.

**Patterns**: Higher-level abstractions built on primitives:
- `ffi::Slice<T>` - Safe slice passing
- `ffi::Option<T>` - Optional values
- `ffi::CStrPtr` - ASCII strings
- `ffi::String` - UTF-8 strings
- `ffi::Vec` - Growable arrays
- Service patterns - Turn Rust traits into class-like interfaces

**Backends**: Transform the `Inventory` into target language bindings. Each backend converts types, functions, and patterns into idiomatic code for that language.

### Reference Project
`crates/reference_project/` contains comprehensive examples of every supported feature. It's used for:
- Testing all backends work correctly
- Demonstrating best practices
- Validating new features

Key modules:
- `functions/` - All function patterns
- `types/` - Type definitions
- `patterns/` - Pattern implementations
- `services/` - Service pattern examples
- `constants.rs` - Constant exports

## Key Patterns

### Adding New FFI Functions
```rust
#[ffi_function]
pub fn my_function(input: MyType) -> Result<Output, Error> {
    // Implementation
}

// Register in inventory
pub fn ffi_inventory() -> Inventory {
    Inventory::new()
        .register(function!(my_function))
        .build()
}
```

### FFI-Safe Types
```rust
#[ffi_type]
pub struct MyStruct {
    pub field: u32,
}

#[ffi_type]
pub enum MyEnum {
    Variant1,
    Variant2(u32),
}
```

### Service Patterns
Services turn Rust traits into class-like interfaces in target languages:
```rust

#[ffi_type(service)]
impl MyService {}

#[ffi_service]
impl MyService {
    pub fn new() -> Result<Self, Error> { /* */ }
    pub fn method(&self) -> u32 { /* */ }
}
```

