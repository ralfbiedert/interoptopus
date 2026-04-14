+++
title = "Services"
weight = 25
+++

Services turn Rust `impl` blocks into class-like objects with constructors, methods, and automatic cleanup. They are the primary mechanism for exposing stateful, object-oriented APIs across the FFI boundary.


In forward interop, a service is defined by marking a struct with `#[ffi(service)]` and annotating its `impl` block with `#[ffi]`. 

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

Any method returning `Self` or `Result<Self, E>` is treated as a constructor. A service can have multiple constructors with different names and parameter lists.

Methods take either `&self`, `&mut self`, or—in the case of async services—`this: Async<Self>` (see below). Methods taking `&mut self` must honor Rust's aliasing guarantees. These methods must not be called in parallel from two different threads.

Service methods can return `ffi::Result<T, Error>` to propagate errors across the FFI boundary. The error type must be an `#[ffi]` enum. In C#, error results are translated into typed exceptions that can be caught with standard `try`/`catch`.


## Async Services

For asynchronous methods, the service derives `AsyncRuntime` and includes a `runtime` field. Async methods take `Async<Self>` instead of `&self`, which provides access to the service and its thread-local runtime context.

```rust
#[ffi(service)]
#[derive(AsyncRuntime)]
pub struct ServiceAsyncBasic {
    runtime: Tokio,
}

#[ffi]
impl ServiceAsyncBasic {
    pub fn create() -> ffi::Result<Self, Error> {
        result_to_ffi(|| {
            let runtime = Tokio::new();
            Ok(Self { runtime })
        })
    }

    pub async fn call(_: Async<Self>) -> ffi::Result<(), Error> {
        ffi::Ok(())
    }
}
```



## Inventory Registration

Services must be registered in the `RustInventory` with the `service!()` macro:

```rust
pub fn inventory() -> RustInventory {
    RustInventory::new()
        .register(service!(ServiceBasic))
        .register(service!(ServiceMultipleCtors))
        .register(service!(ServiceAsyncBasic))
        .validate()
}
```
