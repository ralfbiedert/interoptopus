[![crates.io-badge]][crates.io-url]
[![docs.rs-badge]][docs.rs-url]
![license-badge]
[![rust-version-badge]][rust-version-url]
[![rust-build-badge]][rust-build-url]

# Interoptopus 🐙

Productive, performant, robust interop for Rust. Pick three.


With Interoptopus you can write code like this:

```rust,ignore
#[ffi(service)]
pub struct Hello {}

#[ffi]
impl Hello {
    pub fn world() -> Result<Self, Error> { Ok(Self {}) }
}
```

and then call it from your other language like this:

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

Read our [getting started guide](https://interoptopus.rs/getting-started/).

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
<sup>1</sup> You can implement basic support for a new language in just a few hours, no pull request needed.<br/>


## Performance

In essence, plain calls are near-zero overhead (1-10ns), as are most structs, enums and services. If serialization is involved it scales with payload size. Using the .NET runtime as a plugin adds ~20 MB overhead.  

For more details see [our benchmark numbers here](https://interoptopus.rs/benchmarks).



## Further Reading

- See our [changelog](https://interoptopus.rs/changelog).
- Read the [FAQ](https://interoptopus.rs/faq).


## Contributing

PRs are very welcome!

- Submit small bug fixes directly. Major changes should be issues first.
- New features or patterns must be materialized in the reference project and accompanied by
  at least a C# interop test.

[crates.io-badge]: https://img.shields.io/crates/v/interoptopus.svg
[crates.io-url]: https://crates.io/crates/interoptopus
[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[docs.rs-badge]: https://docs.rs/interoptopus/badge.svg
[docs.rs-url]: https://docs.rs/interoptopus/
[rust-version-badge]: https://img.shields.io/badge/rust-1.93%2B-blue.svg?maxAge=3600
[rust-version-url]: https://github.com/ralfbiedert/interoptopus
[rust-build-badge]: https://github.com/ralfbiedert/interoptopus/actions/workflows/rust.yml/badge.svg
[rust-build-url]: https://github.com/ralfbiedert/interoptopus/actions/workflows/rust.yml
[interoptopus_csharp]: https://docs.rs/interoptopus_csharp
[interoptopus_c]: https://docs.rs/interoptopus_c
[interoptopus_cpython]: https://docs.rs/interoptopus_cpython
