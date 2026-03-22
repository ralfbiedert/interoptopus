//! FFI-safe buffer backing [`Wire<T>`](super::Wire).
//!
//! A `WireBuffer` is the raw storage behind every `Wire<T>`. It tracks a `data` pointer, a
//! `len` (bytes written), and a `capacity`. Dropping it on the Rust side reconstructs and
//! frees the underlying `Vec<u8>`. When returned to a foreign caller, that caller must
//! invoke `interoptopus_wire_destroy` when done.

use std::io::{Read, Write};

/// FFI-safe buffer that can represent owned byte storage.
#[repr(C)]
pub struct WireBuffer {
    data: *mut u8,
    len: i32,
    capacity: i32,
}

unsafe impl Send for WireBuffer {}
unsafe impl Sync for WireBuffer {}

impl WireBuffer {
    /// Create a new owned buffer from a Vec
    #[must_use]
    pub fn from_vec(mut vec: Vec<u8>) -> Self {
        let data = vec.as_mut_ptr();
        let len = i32::try_from(vec.len()).expect("Too large Wire buffer!");
        let capacity = i32::try_from(vec.capacity()).expect("Too large Wire buffer!");

        std::mem::forget(vec); // LEAKS the vec here, must use interoptopus_wire_destroy() to free it

        Self { data, len, capacity }
    }

    /// Create an empty owned buffer with capacity
    #[must_use]
    pub fn with_size(size: usize) -> Self {
        Self::from_vec(vec![0u8; size])
    }

    /// Get length of the buffer
    #[must_use]
    pub fn len(&self) -> usize {
        usize::try_from(self.len).expect("Invalid Wire buffer len")
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
struct WireBufferWriter<'a> {
    buf: &'a mut WireBuffer,
    pos: usize,
}

impl<'a> WireBufferWriter<'a> {
    pub fn new(buf: &'a mut WireBuffer) -> Self {
        Self { buf, pos: 0 }
    }
}

impl std::io::Write for WireBufferWriter<'_> {
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
    buf: &'a WireBuffer,
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

impl Clone for WireBuffer {
    fn clone(&self) -> Self {
        Self::from_vec(self.as_slice().to_vec())
    }
}

impl Drop for WireBuffer {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wire_ownership() {
        let owned_wire_buffer = WireBuffer::with_size(64);
        assert!(owned_wire_buffer.is_owned());
    }

    #[test]
    fn wire_buffer_reader_test() {
        use std::io::Read;

        let data = vec![1, 2, 3, 4, 5];
        let buffer = WireBuffer::from_vec(data.clone());
        let mut reader = buffer.reader();

        // Read partial data
        let mut partial_output = vec![0u8; 3];
        let partial_bytes_read = reader.read(&mut partial_output).unwrap();
        assert_eq!(partial_bytes_read, 3);
        assert_eq!(partial_output, &data[0..3]);

        // Read remaining data
        let mut remaining_output = vec![0u8; 5];
        let remaining_bytes_read = reader.read(&mut remaining_output).unwrap();
        assert_eq!(remaining_bytes_read, 2);
        assert_eq!(&remaining_output[0..2], &data[3..5]);

        // Read again should return 0 (EOF)
        let mut output = vec![0u8; 10];
        let bytes_read = reader.read(&mut output).unwrap();
        assert_eq!(bytes_read, 0);
    }
}
