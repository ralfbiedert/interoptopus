# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Interoptopus is a polyglot bindings generator that enables robust Rust libraries to be easily consumed from other languages. The core philosophy is to generate zero-cost, idiomatic bindings that feel hand-crafted rather than auto-generated.

## Architecture

### Core Components

- **`crates/core`** - Main Interoptopus library with inventory system, FFI abstractions, and type definitions
- **`crates/proc_macros`** - Procedural macros (`#[ffi_type]`, `#[ffi_function]`, etc.)
- **Backend crates** - Language-specific code generators:
  - `crates/backend_csharp` - C# bindings (Tier 1, full feature support)
  - `crates/backend_c` - C header generation (Tier 2)
  - `crates/backend_cpython` - Python bindings using ctypes (Tier 2)
  - `crates/backend_utils` - Shared utilities for backends
- **`crates/reference_project`** - Comprehensive test project showcasing all features
- **`tests/`** - Integration tests and benchmarks for each backend

### Key Patterns

The project follows a pattern-based architecture where common FFI scenarios are abstracted into reusable patterns:
- **Functions**: Regular functions and delegates
- **Types**: Composites, enums, opaques, references
- **Patterns**: ASCII pointers, options, slices, strings, results, callbacks
- **Services**: Object-oriented APIs that become classes in target languages, including async support

## Development Commands

### Building and Testing
```bash
# Build everything
cargo build

# Run all tests (also generates bindings if INTEROPTOPUS_UPDATE_BINDINGS=1)
cargo test

# Update bindings using Just
just update-bindings

# Run specific backend tests
cargo test -p tests --features backend_csharp
```

### Benchmarking
- **C# benchmarks**: `tests/tests/csharp_benchmarks/`
- **Python benchmarks**: `tests/tests/cpython_benchmarks/`

Results show call overhead in nanoseconds for various FFI patterns.

## Working with Backends

### Adding New Features
1. Implement in `crates/reference_project/src/`
2. Add to inventory in `crates/reference_project/src/lib.rs`
3. Ensure C# backend support (minimum requirement)
4. Add integration tests

### Backend Development
- Backends are fully decoupled from the core project
- Copy existing backend as starting point for new languages
- Use `crates/backend_utils` for common functionality
- Template-based generation using Tera

## Code Organization

### Reference Project Structure
- `constants.rs` - Primitive constants and const evaluation results
- `functions/` - Various function signature patterns
- `patterns/` - Common FFI patterns (strings, slices, options, etc.)
- `services/` - Object-oriented service patterns including async
- `types/` - Type definitions (basic, enums, composites, generics)

### Feature Flags
- `derive` - Enables procedural macros (default)
- `serde` - Serde support for internal types
- `log` - Logging on FFI errors

## Testing Strategy

The project uses comprehensive integration testing:
- Generated bindings must compile and pass tests in target languages
- Reference project exercises every feature at least once
- Benchmarks ensure performance expectations are met
- Both sync and async patterns are tested

## Language Support Tiers

- **Tier 1**: C# - Full feature support, production ready
- **Tier 2**: C, Python - May be missing features, contributions welcome

New backends can be added independently without modifying core crates.