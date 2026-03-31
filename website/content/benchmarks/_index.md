+++
title = "Benchmarks"
weight = 200
+++


Generated low-level bindings should be zero cost w.r.t. hand-crafted bindings for that language.
That said, even hand-crafted bindings encounter some target-specific overhead
at the FFI boundary (e.g., marshalling, pinning, and safety checks). For C# that cost
is often nanoseconds, for Python it can be microseconds.


## C#

In essence, plain calls are near-zero overhead. Wire-based (JSON) transfers scale with payload size.
The .NET runtime adds ~20 MB RSS on first plugin load.


### Calling Rust

The 'forward calling mode', i.e., a C# application calling an embedded Rust `.dll`. Used when you
have a legacy app but want high-performance Rust under the hood.

| Construct                                               | ns / call        |
|---------------------------------------------------------|------------------|
| `primitive_void()`                                      | 3                |
| `primitive_u64(0)`                                      | 4                |
| `pattern_delegate_retained(delegate)`                   | 21               |
| `pattern_ascii_pointer("hello world")`                  | 20               |
| `pattern_utf8_string("hello world")`                    | 52               |
| `await serviceAsync.Success()` \[.NET task rescheduled\]| 1835<sup>1</sup> |
| `await serviceAsync.Success()` \[Rust result returend\] | 564              |

<sup>1</sup> Includes .NET wakeup overhead — see the [FAQ](/faq#performance).

### Calling .NET

The 'reverse calling mode', i.e., a Rust application loading a .NET `.dll`. Used when you have a modern
Rust app, but need to rely on legacy .NET libraries.

| Construct                                              | ns / call         |
|--------------------------------------------------------|-------------------|
| `plugin.primitive_void()`                              | 1                 |
| `plugin.primitive_u32(42)`                             | 2                 |
| `plugin.wire_hashmap_string({"foo": "bar"}).unwire()`  | 957               |
| `plugin.wire_hashmap_string(16 x {_16: _16}).unwire()` | 5338              |
| `plugin.add_one(1).await` \[sequential\]               | 1340 <sup>1</sup> |
| `plugin.add_one(1).await` \[64 in-flight\]             | 628               |

<sup>1</sup> Includes kernel wakeup overhead — see the [FAQ](/faq#performance).

### Memory

In reverse mode, loading the .NET runtime and a plugin adds about ~20 MB to the process's memory footprint. Note this heavily depends on what your plugin actually does; the numbers here are for a 'hello world' use case.

| Phase                | RSS (MB) |
|----------------------|----------|
| Pure Rust app        | 4.94     |
| + .NET Runtime       | 5.96     |
| + .NET Plugin Loaded | 24.33    |
| + Method call        | 24.34    |

All benchmarks above were measured on .NET 10 and Windows 11.
