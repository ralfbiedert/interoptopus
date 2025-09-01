use crate::lang::{Composite, Docs, Field, Meta, Primitive, Type, TypeInfo};
use std::io::{Read, Write};
use std::marker::PhantomData;

/// FFI buffer that can represent both owned and borrowed data
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

    /// Check if this buffer owns its data
    #[must_use]
    pub const fn is_owned(&self) -> bool {
        self.capacity != 0
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
        // Explicitly do nothing here as the wire buffer must be deallocated on the other side.
    }
}

unsafe impl TypeInfo for WireBuffer<'_> {
    fn type_info() -> Type {
        let fields = vec![
            Field::new("data".to_string(), Type::ReadPointer(Box::new(Type::Primitive(Primitive::U8)))),
            Field::new("len".to_string(), Type::Primitive(Primitive::I32)),
            Field::new("capacity".to_string(), Type::Primitive(Primitive::I32)),
        ];

        let docs = Docs::from_lines(vec!["FFI buffer for Wire data transfer".to_string()]);
        let composite = Composite::with_meta("WireBuffer".to_string(), fields, Meta::with_docs(docs));

        Type::Composite(composite)
    }
}
