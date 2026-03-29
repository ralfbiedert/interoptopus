+++
title = "Documentation"
sort_by = "weight"
template = "section.html"
weight = 100
+++


Interoptopus allows you to interface your Rust code with other languages. 

Built on top of an interop data model it comes with a rich set of primitives to define meaningful abstractions, and code gen backends to transform these into idiomatic backend code. 

The `#[ffi]` attribute lets you annotate items such as structs, enums, functions or service (impl) blocks; Interoptopus inspects these at compile time, validates that they are FFI-safe, and registers them in an inventory that backends can query.

```rust
#[ffi(service)]
pub struct Hello {}

#[ffi]
impl Hello {
    pub fn world() -> Result<Self, Error> { Ok(Self {}) }
}
```

Backends then consume the inventory and emit native interop glue containing declarations, marshalling helpers, service classes, async trampolines, and more. The generated code is as fast and idiomatic as anything you would have written by hand, without you having to write it. 

All you need to do is call back in:

```cs
var service = Hello.World();
service.Dispose();
```

Interoptopus allows you to focus on meaningful business logic, without wading through weeks of segfaults getting your interop glue performant and safe.

