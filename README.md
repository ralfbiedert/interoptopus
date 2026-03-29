[![crates.io-badge]][crates.io-url]
[![docs.rs-badge]][docs.rs-url]
![license-badge]
[![rust-version-badge]][rust-version-url]
[![rust-build-badge]][rust-build-url]

# Interoptopus 🐙

Productive, performant, robust interop for Rust. Pick three.


With Interoptopus yo can write code like this:

```rust,ignore
#[ffi(service)]
pub struct Hello {}

#[ffi]
impl Hello {
    pub fn world() -> Result<Self, Error> { Ok(Self {}) }
}
```

and then call it from your other language like that:

```csharp
var service = Hello.World();
```

Its key features include:
- Nanosecond fast.
- Supports structs, data-enums, callbacks, services, async, idiomatic error handling, and much more ...
- Bidirectional interop, export your Rust library, or load foreign code into your Rust app. 
- Painless workflow, no external tooling required.
- Polyglot core, first-class support for C#, can support any other language.


## Getting Started 

Read our [documentation here](interoptopus.rs/docs).

## Feature Flags

Gated behind **feature flags**, these enable:

- `macros` - Proc macros such as `#[ffi]`.
- `serde` - Serde attributes on internal types.
- `tokio` - Convenience support for async services via Tokio.
- `unstable-plugins` - Experimental 'reverse interop' plugins. Not semver stable!


## Supported Languages

| Language | Backend Crate                                    |  Status |
|----------|--------------------------------------------------|---------|
| C#       | [**interoptopus_csharp**][interoptopus_csharp]   |  ✅     |
| C        | [**interoptopus_c**][interoptopus_c]             |  ⏯️     |
| Python   | [**interoptopus_cpython**][interoptopus_cpython] |  ⏯️     |
| Other    | Write your own backend<sup>1</sup>               |   |

<sup>✅</sup> Tier 1 target. Active maintenance and production use. Full support of all features.<br/>
<sup>⏯️</sup> Tier 2 target. Currently suspended, contributors wanted!<br/>
<sup>1</sup> You can implement basic support for new language in just a few hours, no pull request needed.<br/>


## Performance

In essence, plain calls are near-zero overhead (1-10ns), as are most structs, enums and services. If serialization is involved it scales with payload size. Using the .NET runtime as a plugin adds ~20 MB overhead.  

For more details see [our benchmarks numbers here](interoptopus.rs/benchmarks).



## Further Reading

- See our [changelog](interoptopus.rs/changelog).
- Read the [FAQ](interoptopus.rs/faq).


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
[rust-version-badge]: https://img.shields.io/badge/rust-1.93%2B-blue.svg?maxAge=3600
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
[real-project-layout]: https://github.com/ralfbiedert/interoptopus/tree/master/examples/production_project
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
