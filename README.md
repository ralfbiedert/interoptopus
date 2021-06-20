
[![Latest Version]][crates.io]
[![docs]][docs.rs]
![MIT]

## Interoptopus

🦀  →  🐙 →  Python, C#, C, ...

FFI bindings to your favorite language. Composable. Explicit. Escape hatches included.


### Overview

If you ...

- wrote an `extern "C"` API in Rust
- need C#, Python, C, ... bindings to your library, all at the same time
- prefer having fine-grained control over your API and interop generation
- would like to use quality-of-life [patterns](crate::patterns) on both sides (e.g., [options](crate::patterns::option), [slices](crate::patterns::slice), '[classes](crate::patterns::class)') where feasible
- create your own bindings for a not-yet supported language
- want all your binding-related information (e.g., documentation) in Rust code

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
pub struct Vec3f32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn my_game_function(input: Option<&Vec3f32>) -> Vec3f32 {
    Vec3f32 { x: 2.0, y: 4.0, z: 6.0 }
}

interoptopus::inventory_function!(ffi_inventory, [], [my_game_function], []);
```

You can now use one of these backends to generate interop code:

| Language | Crate | Sample Output | Comment |
| --- | --- | --- | --- |
| C# (incl. Unity) | [**interoptopus_backend_csharp**](https://crates.io/crates/interoptopus_backend_csharp) | [Interop.cs](https://github.com/ralfbiedert/interoptopus/blob/master/interoptopus_backend_csharp/tests/output/Interop.cs) | Built-in. |
| C | [**interoptopus_backend_c**](https://crates.io/crates/interoptopus_backend_c) | [my_header.h](https://github.com/ralfbiedert/interoptopus/blob/master/interoptopus_backend_c/tests/output/my_header.h) | Built-in.|
| Python [CFFI](https://cffi.readthedocs.io/en/latest/index.html) | [**interoptopus_backend_cpython_cffi**](https://crates.io/crates/interoptopus_backend_cpython_cffi) | [reference.py](https://github.com/ralfbiedert/interoptopus/blob/master/interoptopus_backend_cpython_cffi/tests/output/reference_project.py) | Built-in.  |
| Your language | Write your own backend! | - | See existing backends. |

### Features

See the [reference project](https://github.com/ralfbiedert/interoptopus/tree/master/interoptopus_reference_project/src) for a list of all supported features.

### Current Status

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
