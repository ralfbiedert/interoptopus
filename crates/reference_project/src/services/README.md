In most real-world libraries, services are what you will probably create the most. This folder
shows various ways to author them.

| File                                     | Description                                           |
|------------------------------------------|-------------------------------------------------------|
| [`asynk.rs`](asynk.rs)                   | Transparent async-async calls from C# to Rust.        |
| [`basic.rs`](basic.rs)                   | A very simple service, start here. 🍼                 |
| [`callback.rs`](callback.rs)             | Passing callbacks to and invoking them from services. |
| [`dependent.rs`](dependent.rs)           | How to create services from other services.           |
| [`ignored.rs`](ignored.rs)               | Ignoring methods.                                     |
| [`multiple_ctors.rs`](multiple_ctors.rs) | Providing multiple constructors.                      |
| [`on_panic.rs`](on_panic.rs)             | Specifying panic behavior.                            |
| [`result.rs`](result.rs)                 | Error handling.                                       |
| [`slice.rs`](slice.rs)                   | Sending and receiving slices.                         |
| [`string.rs`](string.rs)                 | UTF8 and ASCII strings.                               |
