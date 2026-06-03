//! Per-thread storage for uncaught exceptions reported from foreign plugins.
//!
//! When a C# trampoline catches an unhandled exception it calls back into Rust via
//! the [`TRAMPOLINE_UNCAUGHT_EXCEPTION`](super::trampoline::TRAMPOLINE_UNCAUGHT_EXCEPTION)
//! callback. The callback stashes the message into a thread-local; every generated
//! `plugin!` method then calls [`panic_on_uncaught`] right after its FFI invocation
//! so the exception surfaces as a Rust panic on the calling thread.
//!
//! Generated wrappers must call [`clear`] before the invocation to drop any stale
//! value left over from a previous panic that was swallowed by `catch_unwind`.

use std::cell::RefCell;

thread_local! {
    static LAST_UNCAUGHT_EXCEPTION: RefCell<Option<String>> = const { RefCell::new(None) };
}

/// Drops any pending uncaught-exception message on the current thread.
pub fn clear() {
    LAST_UNCAUGHT_EXCEPTION.with(|c| {
        c.borrow_mut().take();
    });
}

/// Removes and returns the pending uncaught-exception message for the current thread.
#[must_use]
pub fn take() -> Option<String> {
    LAST_UNCAUGHT_EXCEPTION.with(|c| c.borrow_mut().take())
}

/// Stores `msg` as the current thread's pending uncaught-exception message.
pub fn set(msg: String) {
    LAST_UNCAUGHT_EXCEPTION.with(|c| *c.borrow_mut() = Some(msg));
}

/// Panics with the pending uncaught-exception message, if any.
///
/// Generated `plugin!` method wrappers call this right after invoking the FFI
/// function so that exceptions caught by the plugin's outer try/catch propagate
/// as a Rust panic on the caller's thread.
pub fn panic_on_uncaught() {
    if let Some(msg) = take() {
        panic!("uncaught exception in plugin call: {msg}");
    }
}

/// C ABI callback registered with foreign plugins as the uncaught-exception sink.
///
/// `ctx` is ignored (callers may pass null). `message` is a UTF-8 byte slice of
/// length `len`, not null-terminated.
///
/// # Safety
///
/// `message` must point to at least `len` valid bytes for the duration of the call.
pub unsafe extern "C" fn uncaught_exception_callback(_ctx: *const u8, message: *const u8, len: i32) {
    if message.is_null() || len <= 0 {
        return;
    }
    let bytes = unsafe { std::slice::from_raw_parts(message, len.unsigned_abs() as usize) };
    let msg = String::from_utf8_lossy(bytes).into_owned();
    set(msg);
}

/// Returns the function pointer for [`uncaught_exception_callback`] cast to `*const u8`,
/// suitable for passing to a plugin's trampoline register fn.
#[must_use]
#[allow(clippy::fn_to_numeric_cast_any)]
pub fn callback_ptr() -> *const u8 {
    uncaught_exception_callback as *const u8
}
