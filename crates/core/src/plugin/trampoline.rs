//! Well-known trampoline IDs for reverse-interop (plugin) scenarios.
//!
//! When a Rust host loads a foreign plugin (e.g., a .NET DLL), it needs to
//! provide certain runtime functions (like wire buffer allocation) to the
//! plugin. These are registered via a `_trampoline_register(id, fn_ptr)` call,
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

/// Trampoline ID for the uncaught-exception handler function pointer.
///
/// Signature: `extern "C" fn(ctx: *const u8, message: *const u8, len: i32)`
///
/// Called by the plugin whenever a `[UnmanagedCallersOnly]` trampoline method
/// catches an otherwise-unhandled exception. `ctx` is the opaque context pointer
/// registered via [`TRAMPOLINE_UNCAUGHT_EXCEPTION_CTX`]. `message` is a UTF-8
/// byte slice of length `len` (not null-terminated).
pub const TRAMPOLINE_UNCAUGHT_EXCEPTION: i64 = 0x4952_4F50_5743_0003;

/// Trampoline ID for the uncaught-exception handler context pointer.
///
/// Must be registered alongside [`TRAMPOLINE_UNCAUGHT_EXCEPTION`]. The value
/// is passed back as the first argument (`ctx`) on every invocation.
pub const TRAMPOLINE_UNCAUGHT_EXCEPTION_CTX: i64 = 0x4952_4F50_5743_0004;

/// Query ID for the API guard hash, used with `_trampoline_query_u64`.
pub const QUERY_API_GUARD_HASH: i64 = 0x4952_4F50_5143_0001;
