Advanced "convenience patterns" that will make your life easier, esp. from C# and Python.

| File                            | Description                                                                |
|---------------------------------|----------------------------------------------------------------------------|
| [`api_guard.rs`](api_guard.rs)  | Helper to ensure your bindings match your .DLL.                            |
| [`callback.rs`](callback.rs)    | Callbacks and delegates.                                                   |
| [`option.rs`](option.rs)        | An FFI-safe `Option<>`.                                                    |
| [`primitive.rs`](primitive.rs)  | Other primitives with special handling (e.g., `FFIBool`).                  |
| [`result.rs`](result.rs)        | How to use Rust's `Result<>` over FFI and get exceptions at the other end. |
| [`slice.rs`](slice.rs)          | Receiving slices over FFI.                                                 |
| [`string.rs`](string.rs)        | Passing strings over FFI.                                                  |
| [`surrogate.rs`](surrogates.rs) | Exporting types over FFI you don't control.                                |
| [`services/`](services/)        | How to export "classes" to C# / Python. 🔥                                 |
