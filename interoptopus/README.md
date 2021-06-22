
[![Latest Version]][crates.io]
[![docs]][docs.rs]
![MIT]
[![Rust](https://img.shields.io/badge/rust-1.53%2B-blue.svg?maxAge=3600)](https://github.com/ralfbiedert/interoptopus)
[![Rust](https://github.com/ralfbiedert/interoptopus/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/ralfbiedert/interoptopus/actions/workflows/rust.yml)

## Interoptopus

C#, Python, C, ... → 🐙 → 🦀

FFI from your favorite language to Rust. Escape hatchets included.


### Overview

If you ...

- **have** an `extern "C"` API in Rust, and **need** C#, Python, C, ... bindings to your library,
- prefer having **fine-grained-control** over your API and interop generation,
- would like to use **quality-of-life [patterns](crate::patterns)** on **both sides** (e.g., [options](crate::patterns::option), [slices](crate::patterns::slice), '[classes](crate::patterns::class)') where feasible,
- might need to swiftly **support a new language** (3-4 hours to get basic interop generation for new language),
- think your FFI [**single source of truth**](https://en.wikipedia.org/wiki/Single_source_of_truth) should be living Rust code (incl. documentation),

... then Interoptopus might be for you.


### Known limitations

- not yet used in production
- somewhat verbose if you don't own most of your types (still possible, just more work)
- if you target only a single language and don't care about your FFI layer other solutions might be better


### Example & Backends

Assume you have written this Rust FFI code:

```rust
use interoptopus::{ffi_function, ffi_type};

#[ffi_type]
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn my_function(input: Vec3) -> Vec3 {
    Vec3 { x: 2.0, y: 4.0, z: input.z }
}

interoptopus::inventory_function!(ffi_inventory, [], [my_function], []);
```

You can now use one of these backends to generate interop code:

| Language | Crate | Sample Output |
| --- | --- | --- |
| C# (incl. Unity) | [**interoptopus_backend_csharp**](https://crates.io/crates/interoptopus_backend_csharp) | [Interop.cs](https://github.com/ralfbiedert/interoptopus/blob/master/interoptopus_backend_csharp/tests/output/Interop.cs) |
| C | [**interoptopus_backend_c**](https://crates.io/crates/interoptopus_backend_c) | [my_header.h](https://github.com/ralfbiedert/interoptopus/blob/master/interoptopus_backend_c/tests/output/my_header.h) |
| Python [CFFI](https://cffi.readthedocs.io/en/latest/index.html) | [**interoptopus_backend_cpython_cffi**](https://crates.io/crates/interoptopus_backend_cpython_cffi) | [reference.py](https://github.com/ralfbiedert/interoptopus/blob/master/interoptopus_backend_cpython_cffi/tests/output/reference_project.py) |
| Your language | Write your own backend! | - |

### Features

See the [reference project](https://github.com/ralfbiedert/interoptopus/tree/master/interoptopus_reference_project/src) for a list of all supported features.

### Current Status

- June 20, 2021 - Alpha. Has generated simple working<sup>TM</sup> bindings for a few projects for a week now, many things missing.
- June 13, 2021 - Pre-alpha. Has generated C#, C, Python-CFFI bindings at least once, many things missing, untested.


### FAQ

- [FAQ and Safety Guides](https://github.com/ralfbiedert/interoptopus/blob/master/FAQ.md).

### Contributing

PRs are welcome.

- Bug fixes can be submitted directly. Major changes should be filed as issues
first.

- Anything that would make previously working bindings change behavior or stop compiling
is a major change; which doesn't mean we're opposed to breaking stuff before 1.0, just that
we'd like to talk about it before it happens.

- New features or patterns must be materialized in the reference project and accompanied by
an interop test (i.e., a backend test running C# / Python against a DLL invoking that code)
in at least one included backend.

[Latest Version]: https://img.shields.io/crates/v/interoptopus.svg
[crates.io]: https://crates.io/crates/interoptopus
[MIT]: https://img.shields.io/badge/license-MIT-blue.svg
[docs]: https://docs.rs/interoptopus/badge.svg
[docs.rs]: https://docs.rs/interoptopus/
