//! Well-known trampoline IDs for reverse-interop (plugin) scenarios.
//!
//! When a Rust host loads a foreign plugin (e.g., a .NET DLL), it needs to
//! provide certain runtime functions (like wire buffer allocation) to the
//! plugin. These are registered via a `register_trampoline(id, fn_ptr)` call,
//! where `id` is one of the constants defined here.
//!
//! Use the [`register_wire_trampolines!`](crate::register_wire_trampolines)
//! macro to register the wire buffer trampolines. It invokes `builtins_wire!`
//! internally so the function implementations are shared.

/// Trampoline ID for `interoptopus_wire_create`.
///
/// Signature: `extern "C" fn(size: i32, out_len: *mut i32, out_capacity: *mut i32) -> *mut u8`
pub const TRAMPOLINE_WIRE_CREATE: i64 = 0x4952_4F50_5743_0001;

/// Trampoline ID for `interoptopus_wire_destroy`.
///
/// Signature: `extern "C" fn(data: *mut u8, len: i32, capacity: i32)`
pub const TRAMPOLINE_WIRE_DESTROY: i64 = 0x4952_4F50_5743_0002;
