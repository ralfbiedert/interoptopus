[![crates.io-badge]][crates.io-url]
[![docs.rs-badge]][docs.rs-url]
![license-badge]
[![rust-version-badge]][rust-version-url]
[![rust-build-badge]][rust-build-url]

# Interoptopus 🐙

The polyglot bindings generator for your library.

Write a robust library in Rust, easily access it from your second-favorite language:

- Design a single `.dll` / `.so` in Rust, consume it from anywhere.
- Get `QoL` features (e.g., classes, strings) in languages that have them.
- Painless workflow, no external tooling required.
- Easy to support more languages, backends fully decoupled from main project.

We strive to make our generated bindings _zero cost_. They should be as idiomatic
as you could have reasonably written them yourself, but never magic or hiding the interface
you actually wanted to expose.

## Code you write ...

```rust
use interoptopus::{ffi, function};
use interoptopus::inventory::RustInventory;

#[ffi]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[ffi]
pub fn my_function(input: Vec2) {
    println!("{}", input.x);
}

// List functions you want to export, types are inferred.
pub fn ffi_inventory() -> RustInventory {
    RustInventory::new()
        .register(function!(my_function))
        .validate()
}
```

## ... Interoptopus generates

| Language | Crate                                                                             | Sample Output<sup>1</sup>                                                                                             | Status |
|----------|-----------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------|--------|
| C#       | [**interoptopus_csharp**][interoptopus_csharp]           | [Interop.cs][interop-cs] | ✅      |
| C        | [**interoptopus_c**][interoptopus_c]                     | -                        | ⏯️     |
| Python   | [**interoptopus_cpython**][interoptopus_cpython]         | -                        | ⏯️     |
| Other    | Write your own backend<sup>2</sup>                       | -                        |

<sup>✅</sup> Tier 1 target. Active maintenance and production use. Full support of all features.<br/>
<sup>⏯️</sup> Tier 2 target. Currently suspended, contributors wanted!<br/>
<sup>1</sup> For
the [reference project][reference-project]. <br/>
<sup>2</sup> Add basic support for a new language in just a few
hours. [No pull request needed][new-backends].<br/>

## Getting Started 🍼

If you want to ...

- **get started** see the [**hello world**][hello-world],
- **productify your project**, see the [**real project layout**][real-project-layout],
- **understand what's possible**, see the [**reference project**][reference-project],
- **support a new language**, [**copy the C backend**][backend-c].

## Supported Rust Constructs

See the [**reference project**][reference-project]
for an overview:

- [functions][ref-functions]
  (freestanding functions and delegates)
- [types][ref-types] (composites,
  enums, opaques, references, ...)
- [constants][ref-constants] (
  primitive constants; results of const evaluation)
- [patterns][ref-patterns] (ASCII
  pointers, options, slices, ...)
- [services][ref-services] (turn to
  classes in C# and Python, and async methods)

## Performance 🏁

Generated low-level bindings are _zero cost_ w.r.t. hand-crafted bindings for that language.

That said, even hand-crafted bindings encounter some target-specific overhead
at the FFI boundary (e.g., marshalling, pinning, and safety checks). For C# that cost
is often nanoseconds, for Python it can be microseconds.

For a quick overview, this table lists some common round trip times in _ns / call_:

| Construct                              | C#               |
|----------------------------------------|------------------|
| `primitive_void()`                     | 3                |
| `primitive_u64(0)`                     | 4                |
| `pattern_option(Option.None)`          | 14               |
| `pattern_delegate_adhoc(x => x[0])`    | 477 <sup>1</sup> |
| `pattern_delegate_retained(delegate)`  | 21               |
| `pattern_ascii_pointer("hello world")` | 20               |
| `pattern_utf8_string("hello world")`   | 52               |
| `await serviceAsync.Success()`         | 361 <sup>2</sup> |

<sup>1</sup> First time delegate creation and pinning is expensive in C# (100's of ns). We
recommend you retain the delegate instead for >20x faster
calls, [see for example here][csharp-callbacks].<br/>
<sup>2</sup> Preliminary numbers for full round trip to tokio and back. Although async calls have some intrinsic
overhead
(e.g., spawning a new `TaskCompletionSource` is ~100ns), some of that overhead appears to be a
benchmarking effect when spin-waiting for a newly spawned task. In essence, if your application
benefits from async this overhead is negligible, but simple getters or setters shouldn't needlessly be made async.

<br/>

## Feature Flags

Gated behind **feature flags**, these enable:

- `derive` - Proc macros such as `#[ffi]`.
- `serde` - Serde attributes on internal types.
- `tokio` - Convenience support for async services via Tokio.
- `unstable-plugins` - Experimental 'reverse interop' plugins. Not semver stable!

## Changelog

- **v0.16** - Total rewrite: better architecture,safety, diagnostics.
- **v0.15** - Massive cleanup, bugfix, UX overhaul (+syn2).
- **v0.14** - Better inventory UX.
- **v0.13** - Python backend uses `ctypes` now.

Also see our [upgrade instructions][upgrade-instructions].

## FAQ

- [FAQ and Safety Guides][faq].

## Contributing

PRs are very welcome!

- Submit small bug fixes directly. Major changes should be issues first.
- New features or patterns must be materialized in the reference project and accompanied by
  at least an C# interop test.


[crates.io-badge]: https://img.shields.io/crates/v/interoptopus.svg
[crates.io-url]: https://crates.io/crates/interoptopus
[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[docs.rs-badge]: https://docs.rs/interoptopus/badge.svg
[docs.rs-url]: https://docs.rs/interoptopus/
[rust-version-badge]: https://img.shields.io/badge/rust-1.94%2B-blue.svg?maxAge=3600
[rust-version-url]: https://github.com/ralfbiedert/interoptopus
[rust-build-badge]: https://github.com/ralfbiedert/interoptopus/actions/workflows/rust.yml/badge.svg
[rust-build-url]: https://github.com/ralfbiedert/interoptopus/actions/workflows/rust.yml

[interoptopus_csharp]: https://crates.io/crates/interoptopus_csharp
[interoptopus_c]: https://crates.io/crates/interoptopus_backend_c
[interoptopus_cpython]: https://crates.io/crates/interoptopus_backend_cpython
[interop-cs]: https://github.com/ralfbiedert/interoptopus/blob/master/tests/tests/csharp_reference_project/Interop.cs
[reference-project]: https://github.com/ralfbiedert/interoptopus/tree/master/crates/reference_project/src
[new-backends]: https://github.com/ralfbiedert/interoptopus/blob/master/FAQ.md#new-backends
[hello-world]: https://github.com/ralfbiedert/interoptopus/tree/master/examples/hello_world
[real-project-layout]: https://github.com/ralfbiedert/interoptopus/tree/master/examples/real_project_layout
[backend-c]: https://github.com/ralfbiedert/interoptopus/tree/master/crates/backend_c
[ref-functions]: https://github.com/ralfbiedert/interoptopus/tree/master/crates/reference_project/src/functions
[ref-types]: https://github.com/ralfbiedert/interoptopus/tree/master/crates/reference_project/src/types
[ref-constants]: https://github.com/ralfbiedert/interoptopus/tree/master/crates/reference_project/src/constants.rs
[ref-patterns]: https://github.com/ralfbiedert/interoptopus/tree/master/crates/reference_project/src/patterns
[ref-services]: https://github.com/ralfbiedert/interoptopus/tree/master/crates/reference_project/src/services
[csharp-benchmarks]: https://github.com/ralfbiedert/interoptopus/blob/master/tests/tests/csharp_benchmarks/RESULTS.md
[cpython-benchmarks]: https://github.com/ralfbiedert/interoptopus/blob/master/tests/tests/cpython_benchmarks/RESULTS.md
[csharp-callbacks]: https://github.com/ralfbiedert/interoptopus/blob/master/tests/tests/csharp_reference_project/Test.Pattern.Callbacks.cs
[log-crate]: https://crates.io/crates/log
[upgrade-instructions]: https://github.com/ralfbiedert/interoptopus/blob/master/UPGRADE_INSTRUCTIONS.md
[faq]: https://github.com/ralfbiedert/interoptopus/blob/master/FAQ.md
