Advanced "convenience patterns" that will make your life easier, esp. from C# and Python.

| File                           | Description                                               |
|--------------------------------|-----------------------------------------------------------|
| [`api_guard.rs`](api_guard.rs) | Helper to ensure your bindings match your .DLL.           |
| `ascii_pointer.rs`             | Passing strings over FFI.                                 |
| `callback.rs`                  | Callbacks and delegates.                                  |
| `option.rs`                    | An FFI-safe `Option<>`.                                   |
| `primitives.rs`                | Other primitives with special handling (e.g., `FFIBool`). |
| `result.rs`                    | How to use Rust's `Result<>` over FFI.                    |
| `slice.rs`                     | Receiving slices over FFI.                                |
| `surrogates.rs`                | Exporting types over FFI you don't control.               |
| `services/`                    | How to export "classes" to C# / Python. 🔥                |
