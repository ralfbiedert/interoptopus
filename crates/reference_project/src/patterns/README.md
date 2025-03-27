Advanced "convenience patterns" that will make your life easier, esp. from C# and Python.

| File                           | Description                                                            |
|--------------------------------|------------------------------------------------------------------------|
| [`api_guard.rs`](api_guard.rs) | Helper to ensure your bindings match your .DLL.                        |
| [`callback.rs`](callback.rs)   | Callbacks and delegates.                                               |
| [`option.rs`](option.rs)       | An FFI-safe `ffi::Option`.                                             |
| [`primitive.rs`](primitive.rs) | Other primitives with special handling (e.g., `ffi::Bool`).            |
| [`result.rs`](result.rs)       | How to use `ffi::Result` over FFI and get exceptions at the other end. |
| [`slice.rs`](slice.rs)         | Receiving slices over FFI.                                             |
| [`string.rs`](string.rs)       | Passing strings over FFI.                                              |
| [`surrogate.rs`](surrogate.rs) | Exporting types over FFI you don't control.                            |
| [`vec.rs`](vec.rs)             | Passing high-performance Rust-owned data around.                       |
