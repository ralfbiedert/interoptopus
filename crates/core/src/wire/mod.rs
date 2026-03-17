//! Serialize complex objects into flat byte buffers and transfer them over FFI.
//!
//! # Motivation
//!
//! Not every Rust type is `repr(C)`. Types like `String`, `Vec<T>`, `HashMap<K, V>`,
//! and arbitrary user structs containing them cannot be passed directly through an FFI
//! boundary. `Wire<T>` solves this by serializing the value into a flat byte buffer on
//! one side of the boundary and deserializing it on the other.
//!
//! # How it works
//!
//! A [`Wire<T>`] is a thin `repr(C)` wrapper around a [`WireBuffer`] — a pointer + length
//! + capacity triplet that is safe to pass through `extern "C"` function signatures.
//!
//! ## Lifecycle (Rust → foreign)
//!
//! 1. **Serialize** — call [`Wireable::wire()`] (allocates) or
//!    [`Wireable::wire_with_buffer()`] (borrows caller-supplied memory) to produce a
//!    `Wire<T>` whose buffer contains the serialized bytes.
//! 2. **Transfer** — return the `Wire<T>` from an `extern "C"` function. Because
//!    `Wire<T>` is `repr(C)`, it crosses the FFI boundary as a plain struct copy.
//! 3. **Deserialize** — on the foreign side (e.g., C#), read the buffer bytes and
//!    reconstruct the managed type.
//! 4. **Free** — call `interoptopus_wire_destroy` (emitted by [`builtins_wire!`]) to
//!    drop the Rust-allocated buffer. Borrowed buffers (capacity == 0) are a no-op.
//!
//! ## Lifecycle (foreign → Rust)
//!
//! 1. **Allocate & pin** — on the foreign side, allocate a byte buffer (e.g., C#
//!    `stackalloc`) and pin it so the GC will not move it.
//! 2. **Serialize** — write the managed object into that buffer using the generated
//!    `WireOf*` helper.
//! 3. **Transfer** — pass the `Wire<T>` (with `capacity == 0`, marking it borrowed)
//!    into an `extern "C"` function.
//! 4. **Deserialize** — on the Rust side, call [`Wire::unwire()`] to get the real `T`.
//! 5. **Free** — the foreign side unpins / drops its own buffer.
//!
//! # Wire format
//!
//! All values are written in **little-endian** byte order, sequentially, with no padding
//! or alignment between fields:
//!
//! | Type | Format |
//! |---|---|
//! | `u8`..`u64`, `i8`..`i64`, `f32`, `f64` | Fixed-size little-endian bytes |
//! | `usize` / `isize` | Platform-width little-endian (8 bytes on 64-bit) |
//! | `bool` | 1 byte (`0x00` = false, non-zero = true) |
//! | `String` | `u32` byte-length (LE), then UTF-8 bytes |
//! | `Vec<T>` | `u32` element count (LE), then each element serialized in order |
//! | `HashMap<K,V>` | `u32` entry count (LE), then each key followed by value |
//! | `Option<T>` | 1 byte tag (`0x00` = None, `0x01` = Some), then value bytes if Some |
//! | `(A, B, …)` | Each element serialized in order |
//! | User structs | Each field serialized in declaration order |
//!
//! The wire format is **not self-describing** — both sides must agree on the exact type
//! layout. The `#[ffi]` proc macro generates matching serialization code on both
//! the Rust side ([`WireIO`]) and the foreign side.
//!
//! # Trait overview
//!
//! - [`WireIO`] — serialize, deserialize, and compute byte size. Implemented for
//!   primitives, `bool`, `String`, `Vec`, `HashMap`, `Option`, tuples, and any
//!   user struct annotated with `#[ffi]`.
//! - [`Wireable`] — blanket-implemented for `T: WireIO + 'static`; provides the
//!   convenience methods [`wire()`](Wireable::wire) and
//!   [`wire_with_buffer()`](Wireable::wire_with_buffer).
//! - [`Unwireable`] — blanket-implemented for `Wire<T>`; provides [`unwire()`](Unwireable::unwire).

mod buffer;

pub use buffer::WireBuffer;

use crate::lang::types::SerializationError;
use crate::lang::types::WireIO;
use std::marker::PhantomData;

/// Wraps and transfers complex objects over FFI.
///
/// The backing storage uses a ptr+size representation that can safely cross FFI boundaries.
///
/// # Examples
///
/// ## Creating owned Wire (allocates new buffer):
/// ```rust
/// use interoptopus::wire::Wire;
///
/// // Pre-allocated owned buffer
/// let wire: Wire<String> = Wire::with_size(1024);
/// assert!(wire.is_owned());
/// ```
///
/// ## Creating borrowed Wire (uses external buffer):
/// ```rust
/// use interoptopus::wire::Wire;
///
/// let mut buffer = [0u8; 15];
/// let wire: Wire<String> = Wire::new_with_buffer(&mut buffer);
/// assert!(!wire.is_owned());
/// ```
///
/// ## FFI Usage Example:
/// ```rust
/// use interoptopus::wire::{Wire, Wireable};
///
/// extern "C" fn process_data(mut input: Wire<String>) -> Wire<String> {
///     let string = input.unwire();
///
///     // Process and return new Wire
///     String::from("reply").wire()
/// }
/// ```
///
/// ## Complete C# Integration Example
///
/// When using the C# backend, the following Rust code:
/// ```rust,ignore
/// #[ffi]
/// pub struct UserData {
///     pub name: String,
///     pub age: u32,
///     pub active: bool,
/// }
///
/// #[ffi]
/// pub fn process_user(user: Wire<UserData>) -> Wire<UserData> {
///     let mut data = user.unwire().unwrap();
///     data.age += 1;
///     data.active = true;
///     data.wire()
/// }
/// ```
///
/// Can be used from C# code:
/// ```csharp
/// public static UserData ProcessUser(UserData user)
/// {
///     int bufferSize = user.WireSize();
///     Span<byte> buffer = stackalloc byte[bufferSize];
///
///     // Pin the GC memory so it is not moved while calling Rust.
///     fixed (byte* bufferPtr = buffer)
///     {
///         var wireInput = WireOfUserData.From(user, bufferPtr, bufferSize);
///         var wireResult = process_user(wireInput);
///
///         try
///         {
///             return wireResult.Unwire();
///         }
///         finally
///         {
///             wireResult.Dispose();
///         }
///     }
/// }
/// ```
///
#[repr(C)]
pub struct Wire<'my, T>
where
    T: WireIO + ?Sized,
{
    buf: WireBuffer<'my>,          // FFI-safe storage either owned or borrowed
    _phantom: PhantomData<&'my T>, // behaves like a lifetimed reference
}

impl<'a, T: WireIO> Wire<'a, T> {
    /// Creates a new Wire with owned storage pre-allocated to the given capacity.
    #[must_use]
    pub fn with_size(capacity: usize) -> Wire<'static, T> {
        Wire { buf: WireBuffer::with_size(capacity), _phantom: PhantomData }
    }

    /// Creates a new Wire with borrowed storage from the provided buffer.
    #[allow(clippy::use_self)]
    #[must_use]
    pub fn new_with_buffer(buffer: &'a mut [u8]) -> Wire<'a, T> {
        Wire { buf: WireBuffer::from_slice(buffer), _phantom: PhantomData }
    }

    /// Serialize a value into this Wire's buffer.
    pub fn serialize(&mut self, value: &T) -> Result<(), SerializationError> {
        value.write(&mut self.buf.writer())
    }

    /// Deserialize the value from this Wire's buffer.
    pub fn unwire(&mut self) -> Result<T, SerializationError> {
        T::read(&mut self.buf.reader())
    }

    /// Check if this Wire owns its buffer data.
    #[must_use]
    pub fn is_owned(&self) -> bool {
        self.buf.is_owned()
    }

    /// Get a slice view of the buffer data.
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        self.buf.as_slice()
    }
}

/// Emits and registers helper functions used by [`Wire`](crate::wire::Wire).
#[macro_export]
macro_rules! builtins_wire {
    () => {{
        use ::interoptopus::lang::FunctionInfo;

        #[$crate::ffi_function]
        pub unsafe extern "C" fn interoptopus_wire_destroy(data: *mut u8, len: i32, capacity: i32) {
            if capacity <= 0 {
                // If the buffer was borrowed or allocated on the opposite FFI side, cannot deallocate it.
                return;
            }
            let _ = unsafe { Vec::from_raw_parts(data, usize::try_from(len).expect("Invalid vec length"), usize::try_from(capacity).expect("Invalid vec capacity")) };
        }

        let items = vec![interoptopus_wire_destroy::function_info()];
        let builtins = $crate::pattern::builtins::Builtins::new(items);
        let pattern = $crate::pattern::LibraryPattern::Builtins(builtins);
        $crate::inventory::Symbol::Pattern(pattern)
    }};
}
