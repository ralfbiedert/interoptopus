+++
title = "Forward Interop"
weight = 40
+++

Callable items specific to **exposing Rust to other languages** (e.g., C#) via the `#[ffi]` attribute. Most type, pattern and Wire constructs are supported, for details see the respective `#[ffi]` documentation.


## Functions

Regular freestanding functions are supported, accepting or returning most types:

```rust
#[ffi]
pub fn primitive_void() {}

#[ffi]
pub fn primitive_f64(x: f64) -> f64 { -x }

#[ffi]
pub fn ref_mutable(x: &mut i64) -> &mut i64 { *x = -*x; x }

#[ffi]
pub fn generic(_x: Generic<u32>, _y: Generic<u8, 5>) { }

#[ffi]
pub fn wire(x: Wire<String>) {}
```



## Services

Services turn Rust `impl` blocks into class-like objects with constructors, methods, and automatic cleanup.

### Basic Service

```rust
#[ffi(service)]
pub struct ServiceBasic {}

#[ffi]
impl ServiceBasic {
    pub fn create() -> ffi::Result<Self, Error> {
        ffi::Ok(Self {})
    }
}
```

### Methods & Self

Methods take `&self` or `&mut self`. Return types can be primitives, structs, enums, `ffi::Result`, `ffi::String`, `ffi::Slice`, or `ffi::Vec`.

```rust
#[ffi]
impl ServiceMethods {
    pub fn create() -> ffi::Result<Self, Error> { ffi::Ok(Self::default()) }
    pub fn add(&self, x: u32, y: u32) -> u32 { x + y }
    pub fn set(&mut self, x: u32) { self.value = x; }
}
```

Note that methods taking `&mut self` must still honor Rust's aliasing guarantee. For example, you must not call these methods in parallel from two different threads.


### Multiple Constructors

Any method returning `Self` (or `Result<Self, E>`) is treated as a constructor.

```rust
#[ffi]
impl ServiceMultipleCtors {
    pub fn new_with(some_value: u32) -> ffi::Result<Self, Error> { /* ... */ }
    pub fn new_without() -> ffi::Result<Self, Error> { /* ... */ }
    pub fn new_with_string(_: ffi::CStrPtr) -> ffi::Result<Self, Error> { /* ... */ }
    pub fn new_failing(_: u8) -> ffi::Result<Self, Error> { ffi::Err(Error::Fail) }
}
```

### Service Dependencies

Constructors and methods can take references to other services.

```rust
#[ffi]
impl ServiceDependent {
    pub fn from_main(main: &ServiceMain) -> ffi::Result<Self, Error> {
        ffi::Ok(Self { val: main.val })
    }
    pub fn pass_main(&self, _: &ServiceMain) {}
}
```

### Async Services

Add `#[derive(AsyncRuntime)]` and a `runtime` field, or derive the trait manually. Async methods take `Async<Self>` instead of `&self`, which also provides special access to the thread-local runtime context in runtimes that support it.

```rust
#[ffi(service)]
#[derive(AsyncRuntime)]
pub struct ServiceAsyncBasic {
    runtime: Tokio,
}

#[ffi]
impl ServiceAsyncBasic {
    pub fn create() -> ffi::Result<Self, Error> {
        result_to_ffi(|| Ok(Self { runtime: Tokio::new() }))
    }

    pub async fn call(_: Async<Self>) -> ffi::Result<(), Error> {
        ffi::Ok(())
    }
}
```


### API Guard

We highly recommend you also utilize API guards. When registered via `guard!`, they emit a version check so the foreign side can verify it was generated against the same API.

```rust
#[ffi]
pub fn api_guard() -> Version {
    crate::inventory().into()
}
```


## Inventory Registration

All items must be registered in a `RustInventory`. The inventory uses a builder pattern with dedicated macros:

```rust
pub fn inventory() -> RustInventory {
    RustInventory::new()
        .register(builtins_string!())
        .register(builtins_vec!(u8))
        .register(builtins_vec!(ffi::String))
        .register(guard!(api_guard))
        .register(function!(my_function))
        .register(constant!(MY_CONST))
        .register(extra_type!(Generic<f32>))
        .register(service!(MyService))
        .validate()
}
```

In most cases only functions and services must be registered, types are inferred. 

| Macro | Purpose |
|---|---|
| `function!()` | Register a single `#[ffi]` function |
| `constant!()` | Register a `#[ffi]` constant |
| `service!()` | Register a `#[ffi(service)]` type + its impl block |
| `extra_type!()` | Register a concrete generic specialisation |
| `builtins_string!()` | Register `ffi::String` support functions |
| `builtins_vec!(T)` | Register `ffi::Vec<T>` support functions |
| `builtins_wire!()` | Register `Wire<T>` support functions |
