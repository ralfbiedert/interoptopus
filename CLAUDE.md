# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Interoptopus is a polyglot bindings generator for Rust libraries. It allows you to write a Rust library once and generate idiomatic bindings for multiple target languages (C#, C, Python). The core concept is using FFI-safe Rust code with special attributes (`#[ffi]`) that get processed into language-specific bindings.

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
- `cargo test -p interoptopus_csharp` - Test C# backend (next-gen)
- `cargo check --workspace` - Quick check all crates

## Architecture

### Workspace Structure
```
crates/
├── core/                   # Main interoptopus crate with core FFI abstractions
├── proc_macros/            # Proc macro crate used by core
├── proc_macros_impl/       # Procedural macros (#[ffi_type], #[ffi_function]) incl. testing function
├── backend_c/              # C header generation
├── backend_csharp/         # Next-gen C# backend (experimental)
├── backend_csharp_old/     # Old C# backend
├── backend_cpython/        # Python bindings generation
├── backend_utils/          # Shared utilities for backends
└── reference_project/      # Comprehensive test project using all features
```

Note: `backend_csharp`, `backend_c`, and `backend_cpython` are currently excluded from the workspace while refactoring is in progress. The active workspace members are `core`, `proc_macros`, `proc_macros_impl`, `backend_csharp`, `backend_utils`, and `reference_project`.

### Core Components

**Inventory System** (`crates/core/src/inventory/`): The central registry (`RustInventory`) that tracks all FFI types, functions, constants, and services via HashMaps keyed by unique IDs. Libraries build an inventory using a builder pattern with registration macros like `function!()`, `service!()`, `constant!()`, `extra_type!()`, `builtins_string!()`, `builtins_vec!()`.

**Proc Macros** (`crates/proc_macros_impl/src/`): The `#[ffi]` attribute is the unified entry point that dispatches based on item type — structs/enums go to `types::ffi()`, functions to `function::ffi()`, constants to `constant::ffi()`, and impl blocks to `service::ffi()`.

**FFI Patterns** (`crates/core/src/pattern/`): Higher-level abstractions built on primitives:
- `ffi::Slice<T>` / `ffi::SliceMut<T>` - Safe slice passing
- `ffi::Option<T>` - Optional values
- `ffi::Result<T, E>` - Error handling
- `ffi::CStrPtr` - ASCII strings
- `ffi::String` - UTF-8 strings
- `ffi::Vec<T>` - Growable arrays
- Service patterns - Turn Rust impl blocks into class-like interfaces

**Backends**: Transform the `RustInventory` into target language bindings. Each backend uses the Tera template engine with embedded templates and follows a builder pattern: `Interop::builder().inventory(inv).build()?.write_file("out.cs")?`.

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

### New C# Backend (`backend_csharp`)

The next-gen C# backend uses a **multi-pass pipeline** architecture. Processing happens in `RustLibrary::process()` and flows through two phases, model passes and output passes. Model passes build a well-defined data and interop model iteratively — once done, the resulting interop layer is uniquely specified. The output passes then render the model into a final output format via Tera templates.

**Pass directory structure** (`crates/backend_csharp/src/pass/`):
```
pass/
├── mod.rs              # Shared types (Outcome, PassMeta, PassInfo, MissingItem) + macros
├── meta/
│   └── info.rs         # DLL name, hash, version metadata
├── model/
│   ├── final.rs        # Sentinel pass for convergence
│   ├── fn_map.rs       # Converts function signatures (args + return types)
│   ├── id_maps.rs      # Creates Rust→C# ID mappings for types and functions
│   └── types/
│       ├── blittable.rs  # Determines blittable vs disposable ownership per type
│       ├── kind.rs       # Stores TypeKind per C# type ID + submodule declarations
│       ├── kind/         # Per-category type mapping passes:
│       │   ├── array.rs, delegate.rs, enum.rs, enum_variants.rs,
│       │   ├── opaque.rs, patterns.rs, pointer.rs, primitives.rs,
│       │   ├── service.rs, struct.rs, struct_fields.rs
│       ├── map.rs        # Assembles final Type { name, kind } objects
│       └── names.rs      # Assigns C# type names to all mapped types
└── output/
    ├── enum_ty.rs      # Renders per-enum type definitions via enum_ty.cs template
    ├── enum.rs         # Groups enum renders per output file via enum.cs template
    ├── fn_import.rs    # Renders function import declarations
    ├── header.rs       # File headers
    ├── master.rs       # Determines which files to generate, holds template engine
    └── final.rs        # Assembles everything into a Multibuf (filename→content map)
```

**Model Passes** (iterative, loop until convergence):
All model passes run in a loop via `loop_model_passes_until_done`. Each pass returns `Outcome::Changed` or `Outcome::Unchanged`. The loop repeats until every pass returns `Unchanged`, meaning all dependencies are resolved. This lets passes declare dependencies implicitly — if a type isn't mapped yet, the pass skips it and picks it up on the next iteration. Three macros in `pass/mod.rs` support this: `try_extract_kind!`, `skip_mapped!`, and `try_resolve!`.

Cross-references between passes use path-based types (e.g., `model::types::kind::Pass`, `model::id::Pass`). Each pass file imports `model`/`meta`/`output` from `crate::pass` and uses paths from there.

**Output Passes** (sequential, run once after convergence):
Output passes run after model convergence. They render model data through Tera templates. The pattern is typically two-level: a "type" pass renders individual items (e.g., `enum_ty` renders each enum), then a grouping pass collects results per output file (e.g., `enum` groups all enum renders per `Output`). The `final` pass assembles all intermediary results into the `Multibuf`.

Output passes are held in `IntermediateOutputPasses` (defined in `pipeline/rust/library.rs`) so they can be passed as a group to the final pass.

**Blittability**: The `model::types::blittable` pass computes `Ownership::Blittable` or `Ownership::Disposable` for every type. Primitives, pointers, and delegates are blittable. Composites and enums are blittable if all their fields/variants are. This runs iteratively (convergence loop) so compound types resolve after their dependencies.

**Extension points**: Plugins implement `RustLibraryPlugin` with hooks at `init`, `post_model`, and `post_output`.

**Templates**: `.cs` template files are packed into a tar archive at build time (via `build.rs`), embedded in the binary, and rendered with the Tera engine from `interoptopus_backends`.

## Key Patterns

### Adding New FFI Functions
```rust
#[ffi]
pub fn my_function(input: MyType) -> Result<Output, Error> {
    // Implementation
}

// Register in inventory
pub fn ffi_inventory() -> RustInventory {
    RustInventory::new()
        .register(function!(my_function))
        .validate()
}
```

### FFI-Safe Types
```rust
#[ffi]
pub struct MyStruct {
    pub field: u32,
}

#[ffi]
pub enum MyEnum {
    Variant1,
    Variant2(u32),
}
```

### Service Patterns
Services turn Rust impl blocks into class-like interfaces in target languages:
```rust
#[ffi]
pub struct MyService { /* opaque or transparent fields */ }

#[ffi]
impl MyService {
    pub fn new() -> Result<Self, Error> { /* */ }
    pub fn method(&self) -> u32 { /* */ }
}
```
