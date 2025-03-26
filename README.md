
[![crates.io-badge]][crates.io-url]
[![docs.rs-badge]][docs.rs-url]
![license-badge]
[![rust-version-badge]][rust-version-url]
[![rust-build-badge]][rust-build-url]

## Interoptopus 🐙

The polyglot bindings generator for your library.

Write a robust library in Rust, easily access it from your second-favorite language:

- Design a single `.dll` / `.so` in Rust, consume it from anywhere.
- Get `QoL` features (e.g., classes, strings) in languages that have them.
- Painless workflow, no external tooling required.
- Easy to support more languages, backends fully decoupled from main project.

We strive to make our generated bindings _zero cost_. They should be as idiomatic
as you could have reasonably written them yourself, but never magic or hiding the interface
you actually wanted to expose.


### Code you write ...

```rust
#[ffi_type]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[ffi_function]
pub fn my_function(input: Vec2) {
    println!("{}", input.x);
}

// List functions you want to export, types are inferred.
pub fn ffi_inventory() -> Inventory {
    InventoryBuilder::new()
        .register(function!(my_function))
        .validate()
        .build()
}

```


### ... Interoptopus generates

| Language | Crate | Sample Output<sup>1</sup> | Status |
| --- | --- | --- | --- |
| C# | [**interoptopus_backend_csharp**](https://crates.io/crates/interoptopus_backend_csharp) | [Interop.cs](https://github.com/ralfbiedert/interoptopus/blob/master/tests/tests/csharp_reference_project/Interop.cs) | ✅ |
| C | [**interoptopus_backend_c**](https://crates.io/crates/interoptopus_backend_c) | [my_header.h](https://github.com/ralfbiedert/interoptopus/blob/master/tests/tests/c_reference_project/reference_project.h) | ⏯️ |
| Python  | [**interoptopus_backend_cpython**](https://crates.io/crates/interoptopus_backend_cpython) | [reference.py](https://github.com/ralfbiedert/interoptopus/blob/master/tests/tests/cpython_reference_project/reference_project.py) | ⏯️ |
| Other | Write your own backend<sup>2</sup> | - |

<sup>✅</sup> Tier 1 target. Active maintenance and production use. Full support of all features.<br/>
<sup>⏯️</sup> Tier 2 target. Might be missing features or UX, contributors wanted!<br/>
<sup>1</sup> For the [reference project](https://github.com/ralfbiedert/interoptopus/tree/master/crates/reference_project/src). <br/>
<sup>2</sup> Add basic support for a new language in just a few hours. [No pull request needed](https://github.com/ralfbiedert/interoptopus/blob/master/FAQ.md#new-backends).<br/>


### Getting Started 🍼

If you want to ...
- **get started** see the [**hello world**](https://github.com/ralfbiedert/interoptopus/tree/master/examples/hello_world),
- **productify your project**, see the [**real project layout**](https://github.com/ralfbiedert/interoptopus/tree/master/examples/real_project_layout),
- **understand what's possible**, see the [**reference project**](https://github.com/ralfbiedert/interoptopus/tree/master/crates/reference_project/src),
- **support a new language**, [**copy the C backend**](https://github.com/ralfbiedert/interoptopus/tree/master/crates/backend_c).

### Supported Rust Constructs

See the [**reference project**](https://github.com/ralfbiedert/interoptopus/tree/master/crates/reference_project/src) for an overview:
- [functions](https://github.com/ralfbiedert/interoptopus/tree/master/crates/reference_project/src/functions.rs) (freestanding functions and delegates)
- [types](https://github.com/ralfbiedert/interoptopus/tree/master/crates/reference_project/src/types) (composites, enums, opaques, references, ...)
- [constants](https://github.com/ralfbiedert/interoptopus/tree/master/crates/reference_project/src/constants.rs) (primitive constants; results of const evaluation)
- [patterns](https://github.com/ralfbiedert/interoptopus/tree/master/crates/reference_project/src/patterns) (ASCII pointers, options, slices, classes, ...)


### Performance 🏁

Generated low-level bindings are _zero cost_ w.r.t. hand-crafted bindings for that language.

That said, even hand-crafted bindings encounter some target-specific overhead
at the FFI boundary (e.g., marshalling, pinning, and safety checks). For C# that cost
is often nanoseconds, for Python it can be microseconds.

Detailed call cost tables can be found here: <sup>🔥</sup>

- [**C# call overhead**](https://github.com/ralfbiedert/interoptopus/blob/master/tests/tests/csharp_benchmarks/RESULTS.md)
- [**Python call overhead**](https://github.com/ralfbiedert/interoptopus/blob/master/tests/tests/cpython_benchmarks/RESULTS.md)

For a quick overview, this table lists the most common call types in _ns / call_:

| Construct | C# | Python |
| --- | --- | --- |
| `primitive_void()` | 7 | 272 |
| `primitive_u32(0)` | 8 | 392 |
| `many_args_5(0, 0, 0, 0, 0)` | 10 | 786 |
| `callback(x => x, 0)` | 43 | 1168 |

<br/>



### Feature Flags

Gated behind **feature flags**, these enable:

- `derive` - Proc macros such as `ffi_type`, ...
- `serde` - Serde attributes on internal types.
- `log` - Invoke [log](https://crates.io/crates/log) on FFI errors.


### Changelog

- **v0.15** - Massive cleanup, bugfix, UX overhaul (+syn2).
- **v0.14** - Better inventory UX.
- **v0.13** - Python backend uses `ctypes` now.

Also see our [upgrade instructions](https://github.com/ralfbiedert/interoptopus/blob/master/UPGRADE_INSTRUCTIONS.md).


### FAQ

- [FAQ and Safety Guides](https://github.com/ralfbiedert/interoptopus/blob/master/FAQ.md).


### Contributing

PRs are very welcome!

- Submit small bug fixes directly. Major changes should be issues first.
- New features or patterns must be materialized in the reference project and accompanied by
  at least an C# interop test.

[crates.io-badge]: https://img.shields.io/crates/v/interoptopus.svg
[crates.io-url]: https://crates.io/crates/interoptopus
[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[docs.rs-badge]: https://docs.rs/interoptopus/badge.svg
[docs.rs-url]: https://docs.rs/interoptopus/
[rust-version-badge]: https://img.shields.io/badge/rust-1.83%2B-blue.svg?maxAge=3600
[rust-version-url]: https://github.com/ralfbiedert/interoptopus
[rust-build-badge]: https://github.com/ralfbiedert/interoptopus/actions/workflows/rust.yml/badge.svg
[rust-build-url]: https://github.com/ralfbiedert/interoptopus/actions/workflows/rust.yml
