# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Interoptopus is a polyglot bindings generator for Rust libraries. It allows you to write a Rust library once and generate idiomatic bindings for multiple target languages (C#, C, Python). The core concept is using FFI-safe Rust code with special attributes (`#[ffi]`) that get processed into language-specific bindings.

## Development Commands

### Core Commands
- `just ci` - Run full CI locally (build, fmt check, clippy, test)
- `just build` - Build all workspace crates with all features.
- `just test` - Run all tests with all features via `cargo nextest`
- `just test-agent` - Can be used by agents to test a specific task
- `cargo check --workspace` - Quick check all crates

Agents may modify the `Justfile` section `test-agent` for their needs w.r.t the task at hand.


### Setup
- `just binstall-deps` - Install required tools (`cargo-insta`, `cargo-nextest`) via `cargo-binstall`


## Architecture

### Workspace Structure
```
crates/
├── core/                   # Main interoptopus crate with core FFI abstractions
├── proc_macros/            # Proc macro crate used by core
├── proc_macros_impl/       # Procedural macros (#[ffi_type], #[ffi_function]) incl. testing function
├── backend_csharp/         # Next-gen C# backend (experimental)
├── backend_cpython/        # Python bindings generation (defunct for now)
├── backend_c/              # C header generation (defunct for now)
├── backend_utils/          # Shared utilities for backends
└── reference_project/      # Comprehensive test project using all features
```

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

**Reference Project** (`crates/reference_project/`): Comprehensive examples of every supported feature. Used for:
- Testing all backends work correctly
- Demonstrating best practices
- Validating new features
- Key modules:
  - `functions/` - All function patterns
  - `types/` - Type definitions
  - `patterns/` - Pattern implementations
  - `services/` - Service pattern examples
  - `constants.rs` - Constant exports


### New C# Backend (`backend_csharp`)

The next-gen C# backend uses a **multi-pass pipeline** architecture. 

The backend consists of these major items:

- A `lang` module that defines a base ontology of C# constructs the backend supports and can emit. It is the foundation of model passes and all entities there should be minimal and orthogonal to just express what is needed to define a C# FFI boundary.
- The `pass` module contains model and output passes. 
  - **Model passes** start with core library 'inventory' types and transform them to a well-defined data and interop model based C# `lang` items. Model passes encode the semantics of the interop files to be produced. 
    - Each model pass usually holds part of the transformation ('lang') model. The reason they are broken up into smaller units is to allow for incremental processing, make the logic easier to reason about, and allow for re-use of passes later for other codegen purposes.
    - Model passes should be 'reasonably sized'. If a model pass does or 'knows' too much it should be broken up into smaller passes. Overall passes should not be larger than a few pages of code.  
    - They are iterative and run until convergence (no more changes are made).
    - They can use other model passes by getting or setting data from them (preferably mostly getting), but should not depend on or reuse random helper functions from other passes.
  - **Output passes** take the model and render it into a final output format. Output passes render semantic models into actual, runnable C# code. 
    - They are primarily based on Tera templates
    - They are sequential and run after model passes
    - Output passes might produce several output files, usually reflected as `HashMap<Output, _>`. For example, different types could end up in different `.cs` output files. At the moment this isn't fully supported yet (each pass usually writes all items to every output file, but later we should filter out items that are not needed for a given output)
  - It is imperative that each pass has correct module documentation at the top of the file. 
- Modules under `pipeline` wire up passes in the right order to produce actual output. 

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
│       ├── fallback.rs   # Fallback handling for unmapped types
│       ├── kind.rs       # Stores TypeKind per C# type ID + submodule declarations
│       ├── kind/         # Per-category type mapping passes:
│       │   ├── array.rs, delegate.rs, enum.rs, enum_variants.rs,
│       │   ├── opaque.rs, patterns.rs, pointer.rs, primitives.rs,
│       │   ├── service.rs, struct.rs, struct_fields.rs
│       ├── info/
│       │   ├── managed_conversion.rs  # AsIs/To/Into conversion category per type
│       │   ├── disposable.rs          # Whether a type needs disposal
│       │   └── struct_class.rs        # Whether a composite is a struct or class
│       ├── all.rs        # Container for all C# types (id → Type), the primary query point
│       └── names.rs      # Assigns C# type names to all mapped types
└── output/
    ├── conversion/
    │   ├── unmanaged_conversion.rs  # Conversion suffix/name methods (ToUnmanaged, ToManaged, etc.)
    │   ├── unmanaged_names.rs       # Precomputed unmanaged type names (Name.Unmanaged vs plain)
    │   └── fields.rs               # Per-field custom conversion overrides
    ├── types/
    │   ├── enums/          # Enum rendering: definition, body, body_unmanaged, body_unmanaged_variant,
    │   │                   #   body_to_unmanaged, body_as_unmanaged, body_ctors, body_tostring,
    │   │                   #   body_exception_for_variant, all
    │   ├── composites/     # Composite rendering: definition, body, body_unmanaged,
    │   │                   #   body_to_unmanaged, body_as_unmanaged, all
    │   ├── delegates/      # Delegate rendering: all
    │   └── util/           # Utility types (exceptions, utf8string, etc.)
    ├── fns/
    │   └── import.rs       # Renders function import declarations
    ├── master.rs           # Determines which files to generate, holds template engine
    ├── header.rs           # File headers
    ├── using.rs            # Using directives
    └── final.rs            # Assembles everything into a Multibuf (filename→content map)
```

**Model Passes**:
All model passes run in a loop via `loop_model_passes_until_done`. Each pass returns `Outcome::Changed` or `Outcome::Unchanged`. The loop repeats until every pass returns `Unchanged`, meaning all dependencies are resolved. This lets passes declare dependencies implicitly — if a type isn't mapped yet, the pass skips it and picks it up on the next iteration. Three macros in `pass/mod.rs` support this: `try_extract_kind!`, `skip_mapped!`, and `try_resolve!`.

Cross-references between passes use path-based types (e.g., `model::types::kind::Pass`, `model::id::Pass`). Each pass file imports `model`/`meta`/`output` from `crate::pass` and uses paths from there.

**Output Passes** (sequential, run once after convergence):
Output passes run after model convergence. They render model data through Tera templates. The pattern is typically two-level: a "type" pass renders individual items (e.g., `enum_ty` renders each enum), then a grouping pass collects results per output file (e.g., `enum` groups all enum renders per `Output`). The `final` pass assembles all intermediary results into the `Multibuf`.

Output passes are held in `IntermediateOutputPasses` (defined in `pipeline/rust/library.rs`) so they can be passed as a group to the final pass.

Output passes providing bool values for templates must not do via strings (e.g., `is_foo = "true"`). Instead, tera has an enum Value::Bool for that. 

**Managed Conversion**: The `model::types::info::managed_conversion` pass assigns each type a `ManagedConversion` category (`AsIs`, `To`, or `Into`) that determines how it marshals between managed and unmanaged representations. The `model::types::info::disposable` pass then uses this to determine which types need disposal. The output `conversion/unmanaged_conversion` pass provides suffix/name query methods (e.g., `.ToUnmanaged()`, `.ToManaged()`), and `conversion/unmanaged_names` precomputes the unmanaged type name for each type (plain name if `AsIs`, `Name.Unmanaged` otherwise).

**Extension points**: Plugins implement `RustLibraryPlugin` with hooks at `init`, `post_model`, and `post_output`.

**Templates**: `.cs` template files are packed into a tar archive at build time (via `build.rs`), embedded in the binary, and rendered with the Tera engine from `interoptopus_backends`.


### Reverse Interop / Plugin System

The reverse interop system lets Rust load .NET DLLs as plugins. A Rust `plugin!` macro declares the expected interface (functions + service impl blocks), and the C# backend generates matching `[UnmanagedCallersOnly]` trampolines, interfaces, and types.

**Key crates**:
- `crates/core/src/plugin/` — `service_map.rs` (ServiceHandle, ServiceHandleMap, PluginService traits), `trampoline.rs`
- `crates/proc_macros_impl/src/plugin/` — `model.rs` (parsing), `emit.rs` (code generation)
- `crates/backend_csharp_rt/` — .NET runtime host, loads DLLs at runtime via `hostfxr`
- `crates/backend_csharp/src/pipeline/dotnet/` — the dotnet codegen pipeline

**Service handle model**: Services are opaque types represented by `ServiceHandle<T>` (`#[repr(transparent)]` over `*const T`). This type is `Send + Sync + Copy` and carries `TypeInfo` delegating to `*const T`. On the C# side, a `GCHandle` wraps the managed object and its `IntPtr` is the handle value.

**Proc macro emission** (`emit.rs`): The `plugin!` macro generates:
- A plugin struct holding all fn pointers (bare fns + service methods + destructors)
- `impl Plugin` (symbol loading from DLL)
- `impl PluginInfo` (type/function registration with the inventory)
- Per-service: struct with `handle: ServiceHandle<S>` + method fn ptrs, `impl TypeInfo`, `unsafe impl Send + Sync`, `impl PluginService`, `impl Drop`
- FFI signatures use `ServiceHandle<S>` for service params (by value), with `ServiceHandleMap` for unwrapping through `Result`/`Option` wrappers

**C# codegen for services**: Each service type `Foo` gets:
- A partial class with `struct Unmanaged { IntPtr _handle; }` and conversion methods:
  - `IntoManaged()` — unwrap GCHandle + Free (ownership transfer to managed)
  - `AsManaged()` — unwrap GCHandle without Free (borrow/view)
  - `IntoUnmanaged()` — `GCHandle.Alloc` + `ToIntPtr` (ownership transfer to unmanaged)
  - `AsUnmanaged()` — same as IntoUnmanaged for services
- An interface `IFoo<TSelf>` with `static abstract` methods for ctors (returning `TSelf` or `Task<TSelf>` for async) and instance methods
- Trampoline methods in `static class Interop` that dispatch between FFI and managed code


**Testing reverse interop**: Tests in `crates/backend_csharp/tests/reference_plugins/service.rs`. The `define_plugin!` macro generates C# files, `just build-dotnet-plugins` (or `just _bdp <name>`) compiles them into DLLs placed in `_plugins/`, and `load_plugin!` loads them at runtime. Plugin DLLs need `partial class` on service types to merge with generated partial classes. User implementations live in `Plugin.cs` files under each `<name>.dll/` directory.


## Key Patterns

```rust
// Types — structs and enums use #[ffi] directly
#[ffi]
pub struct MyStruct { pub field: u32 }

#[ffi]
pub enum MyEnum { Variant1, Variant2(u32) }

// Functions
#[ffi]
pub fn my_function(input: MyStruct) -> Result<MyEnum, Error> { /* */ }

// Services — impl blocks become class-like interfaces in target languages
#[ffi]
pub struct MyService { /* opaque or transparent fields */ }

#[ffi]
impl MyService {
    pub fn new() -> Result<Self, Error> { /* */ }
    pub fn method(&self) -> u32 { /* */ }
}

// Inventory registration
pub fn ffi_inventory() -> RustInventory {
    RustInventory::new()
        .register(function!(my_function))
        .register(service!(MyService))
        .validate()
}
```
