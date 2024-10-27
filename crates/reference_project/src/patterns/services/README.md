In most real-world libraries, services are what you will probably create the most. This folder
shows various ways to author them.

| File                                               | Description                                                       |
|----------------------------------------------------|-------------------------------------------------------------------|
| [`basic.rs`](basic.rs)                             | A very simple service, start here. 🍼                             |
| [`callbacks_immediate.rs`](callbacks_immediate.rs) | Passing callbacks to and immediately invoking them from services. |
| [`callbacks_table.rs`](callbacks_table.rs)         | Storing callbacks inside services and invoking them later.        |
| [`ignored.rs`](ignored.rs)                         | Ignoring methods.                                                 |
| [`lifetimes.rs`](lifetimes.rs)                     | Services utilizing lifetimes. Slightly dangerous ⚠️               |
| [`multiple_ctors.rs`](multiple_ctors.rs)           | Providing multiple constructors.                                  |
| [`on_panic.rs`](on_panic.rs)                       | Specifying panic behavior.                                        |
| [`slices.rs`](slices.rs)                           | Sending and receiving slices.                                     |
| [`strings.rs`](strings.rs)                         | Sending and receiving strings.                                    |
