//! Serialize complex objects into flat byte buffers and transfer them over FFI.
//!
//! Not every Rust type is `repr(C)`. Types like `String`, `Vec<T>`, `HashMap<K, V>`,
//! and arbitrary user structs containing them cannot be passed directly through an FFI
//! boundary. `Wire<T>` solves this by serializing the value into a flat byte buffer on
//! one side of the boundary and deserializing it on the other.
//!
//! # How it works
//!
//! A [`Wire<T>`] is essentially a serialized buffer that is safe to pass through
//! FFI boundaries.
//!
//! ### Rust -> Foreign
//!
//! 1. **Serialize** — [`Wire::from`] (or [`Wire::try_from`]) serializes the value into a new Rust-allocated buffer.
//! 2. **Transfer** — the `Wire<T>` is returned from an `#[ffi]` function; as a `repr(C)` struct it crosses the FFI boundary by value.
//! 3. **Deserialize** — the foreign side (e.g., C#) reads the buffer bytes and reconstructs the managed type.
//! 4. **Free** — the foreign side calls `Dispose()` or similar on the wire object, which invokes `interoptopus_wire_destroy`
//!    (emitted by `builtins_wire!`) to drop the Rust-allocated buffer.
//!
//! ### Foreign -> Rust
//!
//! 1. **Allocate** — the generated `WireOf*.From(value)` helper calls `interoptopus_wire_create` (emitted by
//!    `builtins_wire!`) so that Rust allocates the buffer; the foreign side never allocates directly.
//! 2. **Serialize** — the value is serialized into that Rust-allocated buffer.
//! 3. **Transfer** — the `Wire<T>` is passed into an `#[ffi]` function. Rust receives ownership.
//! 4. **Deserialize** — [`Wire::unwire`] or [`Wire::try_unwire`] reads `T` from the buffer.
//! 5. **Free** — Rust drops the `Wire<T>` when the function returns, freeing the buffer.
//!
//! # Example
//!
//! Here we use an actual `HashMap<String, String>` from `std` to demonstrate the transfer of
//! a complex object over FFI.
//!
//! ```rust,ignore
//! # use std::collections::HashMap;
//! # use interoptopus::wire::Wire;
//! #[ffi]
//! fn call_with_string(_input: Wire<HashMap<String, String>>) { }
//! ```
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
//! The wire format is not self-describing, both sides must agree on the exact type
//! layout. The `#[ffi]` proc macro generates matching serialization code on both
//! the Rust side ([`WireIO`]) and the foreign side.
//!
mod buffer;

pub use buffer::WireBuffer;

use crate::bad_wire;
use crate::inventory::{Inventory, TypeId};
use crate::lang::meta::{common_or_module_emission, Docs, Visibility};
use crate::lang::types::{SerializationError, Type, TypeInfo, TypeKind, TypePattern, WireIO};
use std::marker::PhantomData;

/// Wraps and transfers complex objects over FFI.
///
/// The backing storage uses a (ptr, size) representation that can safely cross
/// FFI boundaries.
///
#[repr(C)]
pub struct Wire<'my, T>
where
    T: ?Sized,
{
    buf: WireBuffer<'my>,          // FFI-safe storage either owned or borrowed
    _phantom: PhantomData<&'my T>, // behaves like a lifetimed reference
}

impl<T: TypeInfo + WireIO> Wire<'_, T> {
    /// Serialize `value` into a new owned [`Wire`].
    ///
    /// # Errors
    /// Returns [`SerializationError`] if `value` cannot be serialized into the buffer.
    pub fn try_from(value: T) -> Result<Self, SerializationError> {
        let size = value.live_size();
        let mut buf = WireBuffer::with_size(size);
        value.write(&mut buf.writer())?;
        Ok(Self { buf, _phantom: PhantomData })
    }

    /// Serialize `value` into a new owned [`Wire`].
    ///
    /// # Panics
    /// Panics at compile time if `T::WIRE_SAFE` is false.
    pub fn from(value: T) -> Self {
        const { assert!(T::WIRE_SAFE) }
        let size = value.live_size();
        let mut buf = WireBuffer::with_size(size);
        value.write(&mut buf.writer()).expect("Types with T::WIRE_SAFE must be wirable!");
        Self { buf, _phantom: PhantomData }
    }

    /// Deserialize the value from this Wire's buffer.
    ///
    /// # Errors
    ///
    /// Returns [`SerializationError`] if the buffer contents cannot be deserialized
    /// into `T` (e.g., truncated buffer, malformed data).
    pub fn try_unwire(&mut self) -> Result<T, SerializationError> {
        T::read(&mut self.buf.reader())
    }

    /// Deserialize the value from this Wire's buffer.
    ///
    /// # Panics
    ///
    /// Panics at compile time if `T::WIRE_SAFE` is false.
    pub fn unwire(&mut self) -> T {
        const { assert!(T::WIRE_SAFE) }
        T::read(&mut self.buf.reader()).expect("Types with T::WIRE_SAFE must be un-wirable!")
    }
}

impl<T: TypeInfo + WireIO> TypeInfo for Wire<'_, T> {
    const WIRE_SAFE: bool = false;
    const RAW_SAFE: bool = true;
    const ASYNC_SAFE: bool = true;
    const SERVICE_SAFE: bool = true;
    const SERVICE_CTOR_SAFE: bool = true;

    fn id() -> TypeId {
        TypeId::new(0xE9EF32647BF9C7A70889DC642B63FAC9).derive_id(T::id())
    }

    fn kind() -> TypeKind {
        TypeKind::TypePattern(TypePattern::Wire(T::id()))
    }

    fn ty() -> Type {
        let t = T::ty();
        Type {
            name: format!("Wire<{}>", t.name),
            visibility: Visibility::Public,
            docs: Docs::empty(),
            emission: common_or_module_emission(&[t.emission]),
            kind: Self::kind(),
        }
    }

    fn register(inventory: &mut impl Inventory) {
        T::register(inventory);
        inventory.register_type(Self::id(), Self::ty());
    }
}

impl<T: WireIO> WireIO for Wire<'_, T> {
    fn write(&self, _: &mut impl std::io::Write) -> Result<(), SerializationError> {
        bad_wire!()
    }

    fn read(_: &mut impl std::io::Read) -> Result<Self, SerializationError> {
        bad_wire!()
    }

    fn live_size(&self) -> usize {
        bad_wire!()
    }
}

/// Emits and registers helper functions used by [`Wire`](crate::wire::Wire).
#[macro_export]
macro_rules! builtins_wire {
    () => {{
        #[$crate::ffi]
        pub fn interoptopus_wire_create(size: i32, out_len: &mut i32, out_capacity: &mut i32) -> *mut u8 {
            if size <= 0 {
                *out_len = 0;
                *out_capacity = 0;
                return ::std::ptr::null_mut();
            }
            let size = usize::try_from(size).expect("Invalid Wire buffer size");
            let mut vec: Vec<u8> = vec![0u8; size];
            let data = vec.as_mut_ptr();
            *out_len = i32::try_from(vec.len()).expect("Too large Wire buffer");
            *out_capacity = i32::try_from(vec.capacity()).expect("Too large Wire buffer");
            ::std::mem::forget(vec);
            data
        }

        #[$crate::ffi]
        pub fn interoptopus_wire_destroy(data: *mut u8, len: i32, capacity: i32) {
            if capacity <= 0 {
                return;
            }
            let _ = unsafe { Vec::from_raw_parts(data, usize::try_from(len).expect("Invalid vec length"), usize::try_from(capacity).expect("Invalid vec capacity")) };
        }

        |x: &mut $crate::inventory::RustInventory| {
            <interoptopus_wire_create as $crate::lang::function::FunctionInfo>::register(x);
            <interoptopus_wire_destroy as $crate::lang::function::FunctionInfo>::register(x);
        }
    }};
}
