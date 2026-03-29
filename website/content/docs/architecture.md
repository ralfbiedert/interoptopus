+++
title = "Architecture"
weight = 20
+++


This page gives a high level overview how Interoptopus works under the hood, vaguely in order of end-to-end data flow.


## Proc Macros Transformation

When you annotate an item with `#[ffi]`, a proc macro records its layout or call information at compile time and adds `const {}` checks that verify it is actually FFI-safe, catching many invalid constructs straight in your IDE. 

Complex patterns like services (`#[ffi]` impl blocks on opaque structs) get wrapped with FFI-safe constructor, method, and destructor trampolines. Async methods get callback-based trampolines that integrate with the target language's async runtime.

All of these eventually feed into an inventory.

The `plugin!` macro works in reverse: instead of exporting Rust items for foreign callers, it generates Rust code that calls into a foreign assembly. It emits structs with function pointer fields, a generic runtime loader stub, and wrappers with method dispatchers. 


## Inventory 

The inventory is the data model describing the full FFI surface: types, functions, and higher-level patterns (idiomatic constructs that can be lowered to and restored from a C ABI).

Every item in the inventory is keyed by a deterministic `Id`. They also come with corresponding info structs that can be inspected or modified, for example by codegen plugins that want to rename types, add attributes, or suppress certain items before code generation begins.

The inventory is then handed to a backend. 

## Backends

Backends produce code. 

They first transform the language-neutral inventory into a language-specific data model. This is done through two stages of passes.

Model passes run iteratively in a loop until convergence. Each pass is a small, focused transformation: one pass maps Rust type IDs to C# type IDs, another might determine whether each type is a struct, enum, delegate, or pattern type, or whether a composite should be a `struct` or `class`. 

Output passes run once the model has converged. They take the final model and render it into code fragments that are subsequently aggregated by higher-level passes. 

A final assembly pass stitches everything together into an output buffer that can be written to disk.


## Plugins: Implementing Foreign Code

For reverse interop (plugins), backends generate stub files for the user to fill out, in the C# case a `Plugin.cs`, and various interop glue files containing the required trampolines and scaffolding. Once the stub is implemented it can be compiled (in the .NET case into a DLL or AOT native binary). 

The resulting artifact is what can be loaded from Rust at runtime.


## Loading it

For forward interop the generated code usually loads automatically, or comes with a simple helper. In the C# case the generated bindings use the .NET `[LibraryImport]` attribute; users only need to make the Rust `.dll`/`.so`/`.dylib` available on the library search path — the .NET runtime resolves symbols automatically.

For reverse interop (Rust calling a plugin), a backend-specific runtime loader is used. It resolves function pointers declared by the `plugin!` macro and returns a fully-initialized plugin struct. 

In both directions, the generated helper code handles all marshalling, callers on both sides see idiomatic types.


## Handling Rust Safety

We mostly describe interop bindings as robust (vs. safe). Most other languages can't match or mirror Rust's safety guarantees, and creating APIs that are fully safe in each target language would often mean heavy slowdown. We therefore emit bindings that are resistant to abuse in the target language, but without the extensive safety guarantees a pure rust app would bring. 
