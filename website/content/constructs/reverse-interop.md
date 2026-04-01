+++
title = "Plugins (Reverse Interop)"
weight = 50
+++

Callable items specific to **loading foreign code into Rust** (e.g., C# DLLs as plugins) via the `plugin!` macro. Most type, pattern and Wire constructs are supported, for details see the respective `plugin!` documentation.


## The Plugin Macro

The `plugin!` macro declares an interface for plugins to implement. The macro generates a plugin struct with all necessary FFI glue, symbol loading, API-guard verification, and instrumentation.

```rust
interoptopus::plugin!(MyPlugin {
    fn some_function(x: u32) -> u32;

    impl SomeService {
        fn create() -> Self;
        fn call(&self, x: u32) -> u32;
    }
});
```

The generated `MyPlugin` struct holds function pointers for every declared item. Loading a plugin resolves these symbols from a foreign DLL at runtime.


## Bare Functions

Bare functions live on the plugin directly. They are considered free-standing functions on the foreign side, and become instance methods on the Rust plugin struct.

```rust
interoptopus::plugin!(MyPlugin {
    fn primitive_void();
    fn primitive_u8(x: u8) -> u8;
    fn panic_with_result() -> ffi::Result<(), Error>;
});
```

This could, for example, be called as:

```rust
my_plugin.primitive_void();
```


## Async Functions

Functions marked `async` generate a future-returning wrapper, allowing for transparent async-async calls:

```rust
interoptopus::plugin!(ServiceAsync {
    async fn call_void();
    async fn add_one(x: u32) -> u32;
});
```

This can be called as:

```rust
plugin.call_void().await;
```

Each async call returns an `impl Future`. The future holds a `TaskHandle`, dropping it cancels the operation on the foreign side.


## Services

Inside plugins `impl` blocks can be used to get class-like objects with constructors, methods, and automatic `Drop` support.

```rust
interoptopus::plugin!(MyPlugin {
    impl ServiceA {
        fn create() -> Self;
        fn call(&self, x: u32) -> u32;
    }
});
```
Service constructors become inherent methods on the plugin:

```rust
let svc = plugin.service_a_create();
svc.call(123);
```


## API Guard Verification

The generated code includes an API hash check. If the plugin was compiled against a different version of the interface, loading will fail.

