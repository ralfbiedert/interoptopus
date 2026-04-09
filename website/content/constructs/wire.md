+++
title = "Wire"
weight = 30
+++

The helper `Wire<T>` lets you pass types that aren't normally FFI-compatible. Instead of requiring a strict memory layout, the value is serialized into a buffer on one side and deserialized on the other.

This means you can define plain Rust types with idiomatic fields and still use them in interop.

```rust
#[ffi]
pub struct UserProfile {
    name: String,
    tags: Vec<String>,
    metadata: HashMap<String, String>,
}

#[ffi]
pub fn accept_profile(input: Wire<UserProfile>) { /* ... */ }
```
Note that `Wire<T>` can never be used inside fields, it can only be used in signatures. However, you can freely nest types that require wiring. 

```rust
#[ffi]
pub struct ContainsUserProfile {
    profile: UserProfile,
    more_tags: Vec<String>,
}

#[ffi]
pub fn return_profile() -> Wire<ContainsUserProfile> { /* ... */ }
```

Wiring is supported for types such as `std::string::String`, `std::vec::Vec`, `std::collections::HashMap`, `std::option::Option`, and their composites. We call types that require wiring _wire-only_, and they are infectious for the types they are contained in.

