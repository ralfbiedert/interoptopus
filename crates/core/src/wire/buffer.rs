//! FFI-safe buffer backing [`Wire<T>`](super::Wire).
//!
//! A `WireBuffer` is the raw storage behind every `Wire<T>`. It tracks a `data` pointer, a
//! `len` (bytes written), and a `capacity`. The `capacity` field doubles as an ownership
//! discriminant:
//!
//! - **`capacity > 0`** — the buffer owns a Rust `Vec<u8>` allocation (created via
//!   `interoptopus_wire_create` or [`WireBuffer::with_size`]). Dropping it on the Rust side
//!   reconstructs and frees the Vec. When returned to a foreign caller, that caller must
//!   invoke `interoptopus_wire_destroy` when done.
//! - **`capacity == 0`** — the buffer is empty or borrows externally-managed memory.
//!   Dropping it is a no-op.

use std::io::{Read, Write};
use std::marker::PhantomData;

/// FFI-safe buffer that can represent both owned and borrowed byte storage.
#[repr(C)]
pub struct WireBuffer<'a> {
    data: *mut u8,
    len: i32,
    capacity: i32,
    _phantom: PhantomData<&'a [u8]>,
}

unsafe impl Send for WireBuffer<'_> {}
unsafe impl Sync for WireBuffer<'_> {}

impl<'a> WireBuffer<'a> {
    /// Create a new owned buffer from a Vec
    #[must_use]
    pub fn from_vec(mut vec: Vec<u8>) -> WireBuffer<'static> {
        let data = vec.as_mut_ptr();
        let len = i32::try_from(vec.len()).expect("Too large Wire buffer!");
        let capacity = i32::try_from(vec.capacity()).expect("Too large Wire buffer!");

        std::mem::forget(vec); // LEAKS the vec here, must use interoptopus_wire_destroy() to free it

        WireBuffer { data, len, capacity, _phantom: PhantomData }
    }

    /// Create a new borrowed buffer from a slice
    #[allow(clippy::use_self, reason = "We want to keep the explicit lifetime")]
    #[must_use]
    pub fn from_slice(slice: &'a mut [u8]) -> WireBuffer<'a> {
        WireBuffer {
            data: slice.as_mut_ptr(),
            len: i32::try_from(slice.len()).expect("Too large Wire buffer!"),
            capacity: 0, // indicates borrowed
            _phantom: PhantomData,
        }
    }

    /// Create an empty owned buffer with capacity
    #[must_use]
    pub fn with_size(size: usize) -> WireBuffer<'static> {
        WireBuffer::from_vec(vec![0u8; size])
    }

    /// Get length of the buffer
    #[must_use]
    pub fn len(&self) -> usize {
        usize::try_from(self.len).expect("Invalid Wire buffer len")
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns `true` if this buffer owns its data (Rust-allocated, `capacity > 0`).
    #[must_use]
    pub const fn is_owned(&self) -> bool {
        self.capacity > 0
    }

    /// Get a slice view of the buffer
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(self.data.cast_const(), self.len()) }
        }
    }

    /// Get a mutable slice access to the buffer
    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { std::slice::from_raw_parts_mut(self.data, self.len()) }
        }
    }

    #[must_use]
    pub fn reader(&self) -> impl Read {
        WireBufferReader::new(self)
    }

    #[must_use]
    pub fn writer(&mut self) -> impl Write + '_ {
        WireBufferWriter::new(self)
    }
}

/// Allows serializing types into a wire buffer storage.
struct WireBufferWriter<'a, 'b> {
    buf: &'a mut WireBuffer<'b>,
    pos: usize,
}

impl<'a, 'b> WireBufferWriter<'a, 'b> {
    pub fn new(buf: &'a mut WireBuffer<'b>) -> Self {
        Self { buf, pos: 0 }
    }
}

impl std::io::Write for WireBufferWriter<'_, '_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let data = self.buf.as_slice_mut();
        let remaining = data.len().saturating_sub(self.pos);
        let to_copy = std::cmp::min(remaining, buf.len());

        if to_copy > 0 {
            data[self.pos..self.pos + to_copy].copy_from_slice(&buf[..to_copy]);
            self.pos += to_copy;
        }

        Ok(to_copy)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// Allows deserializing types from a wire buffer storage.
/// This is NOT a zerocopy implementation.
struct WireBufferReader<'a> {
    buf: &'a WireBuffer<'a>,
    pos: usize,
}

impl<'a> WireBufferReader<'a> {
    pub fn new(buf: &'a WireBuffer) -> Self {
        Self { buf, pos: 0 }
    }
}

impl std::io::Read for WireBufferReader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let data = self.buf.as_slice();
        let remaining = data.len().saturating_sub(self.pos);
        let to_copy = std::cmp::min(remaining, buf.len());

        if to_copy > 0 {
            buf[..to_copy].copy_from_slice(&data[self.pos..self.pos + to_copy]);
            self.pos += to_copy;
        }

        Ok(to_copy)
    }
}

impl Drop for WireBuffer<'_> {
    fn drop(&mut self) {
        // Free owned buffers. When a Wire<T> is returned over FFI, the value is moved
        // (no Drop runs on the Rust side), so the other side frees it via
        // `interoptopus_wire_destroy`. This Drop only fires when a Wire is created and
        // dropped on the Rust side without being passed over FFI.
        if self.is_owned() && !self.data.is_null() {
            unsafe {
                let _ = Vec::from_raw_parts(
                    self.data,
                    usize::try_from(self.len).expect("Invalid Wire buffer len"),
                    usize::try_from(self.capacity).expect("Invalid Wire buffer capacity"),
                );
            }
        }
    }
}
