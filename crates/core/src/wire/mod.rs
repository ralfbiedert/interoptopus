//! Serialize complex objects into flat byte buffers and transfer them over FFI.
//!
//! Types like `String`, `Vec<T>`, `HashMap<K, V>`, and structs containing them cannot be
//! passed directly over an FFI boundary. Their in-memory form is a small header (pointer,
//! length, capacity) into a Rust-managed heap allocation — the foreign side cannot read,
//! resize, or free that memory. `Wire<T>` solves this by serializing the value into a flat
//! byte buffer on one side and deserializing it on the other.
//!
//! # Examples
//!
//! ### Accepting a complex argument
//!
//! ```
//! use interoptopus::ffi;
//! use interoptopus::wire::Wire;
//! use std::collections::HashMap;
//!
//! #[ffi]
//! pub fn lookup(mut map: Wire<HashMap<String, String>>) -> u32 {
//!     let map = map.unwire();
//!     map.len() as u32
//! }
//! ```
//!
//! ### Returning a complex value
//!
//! ```
//! use interoptopus::ffi;
//! use interoptopus::wire::Wire;
//!
//! #[ffi]
//! pub fn greeting() -> Wire<String> {
//!     Wire::from("hello".to_string())
//! }
//! ```
//!
//! ### Structs containing non-FFI types
//!
//! Any `#[ffi]` struct whose fields are not all `repr(C)` can be wrapped in
//! `Wire<T>`. The proc macro generates matching serialization code on both
//! sides.
//!
//! ```
//! use interoptopus::ffi;
//! use interoptopus::wire::Wire;
//!
//! #[ffi]
//! pub struct UserProfile {
//!     pub name: String,
//!     pub tags: Vec<String>,
//! }
//!
//! #[ffi]
//! pub fn accept_profile(mut profile: Wire<UserProfile>) {
//!     let profile = profile.unwire();
//!     println!("{}: {:?}", profile.name, profile.tags);
//! }
//! ```
//!
//! ### Deeply nested types
//!
//! `Wire<T>` handles arbitrarily nested structures, including `Vec`, `HashMap`,
//! and `Option` at any depth:
//!
//! ```
//! use interoptopus::ffi;
//! use interoptopus::wire::Wire;
//! use std::collections::HashMap;
//!
//! #[ffi]
//! pub struct Inner { pub score: u32 }
//!
//! #[ffi]
//! pub struct Outer {
//!     pub items: HashMap<u32, Vec<Inner>>,
//! }
//!
//! #[ffi]
//! pub fn process(mut data: Wire<Outer>) -> u32 {
//!     let data = data.unwire();
//!     data.items.values().flatten().map(|i| i.score).sum()
//! }
//! ```
//!
//! ### Registering the helpers
//!
//! Every crate that uses `Wire<T>` must call [`builtins_wire!`](crate::builtins_wire)
//! in its inventory so the create/destroy helpers are available to the foreign side:
//!
//! ```
//! use interoptopus::ffi;
//! use interoptopus::wire::Wire;
//! use interoptopus::{builtins_wire, function};
//! use interoptopus::inventory::RustInventory;
//!
//! #[ffi]
//! pub fn greeting() -> Wire<String> {
//!     Wire::from("hello".to_string())
//! }
//!
//! pub fn ffi_inventory() -> RustInventory {
//!     RustInventory::new()
//!         .register(function!(greeting))
//!         .register(builtins_wire!())
//!         .validate()
//! }
//! ```
//!
//! # Wire vs. Protobuf
//!
//! The natural alternative to `Wire<T>` for passing complex 'variably-sized' types over FFI is
//! [Protocol Buffers](https://protobuf.dev/). Protobuf works, but it comes with significant
//! friction: you need to maintain `.proto` schema files alongside your Rust types, install and
//! run an external code generator as part of your build, integrate that generator into both the Rust
//! and the foreign-language project, and keep all three in sync whenever a type changes.
//! The result is a more complex project setup with more moving parts — and you still have to
//! wire the generated types into your FFI layer by hand.
//!
//! `Wire<T>` eliminates all of that. Types are defined once in Rust with `#[ffi]`, and both
//! the serialization logic and the foreign-language deserialization code are generated
//! automatically as part of the normal interoptopus build. There are no `.proto` files, no
//! external tools, and no schema drift.
//!
//! Beyond ergonomics, `Wire<T>` is in most cases also faster. Because both sides share the
//! exact same compiled type layout, there is no field-tag overhead, no varint encoding, and
//! no dynamic dispatch — just a straight sequential read/write of the exact bytes needed.
//! In our benchmarks, `Wire<T>` usually outperformed Protobuf by roughly 20–200%
//! depending on the payload shape.
//!
//! ![wire-vs-protobuf](https://media.githubusercontent.com/media/ralfbiedert/interoptopus/static/2026-03-protobuf/gfx/wire_vs_protobuf_complex.png)
//!
//! Note, in the benchmarks above, Protobuf was given a slight advantage over `Wire<T>` by not having to
//! FFI allocate. This made Protobuf's performance look slightly better, but would make it unsuitable for
//! `async` use.
//!
//! # Under the Hood
//!
//! A [`Wire<T>`] is essentially a serialized buffer that is safe to pass through
//! FFI boundaries.
//!
//! ## Rust -> Foreign
//!
//! 1. **Serialize** — [`Wire::from`] (or [`Wire::try_from`]) serializes the value into a new Rust-allocated buffer.
//! 2. **Transfer** — the `Wire<T>` is returned from an `#[ffi]` function; as a `repr(C)` struct it crosses the FFI boundary by value.
//! 3. **Deserialize** — the foreign side (e.g., C#) reads the buffer bytes and reconstructs the managed type.
//! 4. **Free** — the foreign side calls `Dispose()` or similar on the wire object, which invokes `interoptopus_wire_destroy`
//!    (emitted by `builtins_wire!`) to drop the Rust-allocated buffer.
//!
//! ## Foreign -> Rust
//!
//! 1. **Allocate** — the generated `WireOf*.From(value)` helper calls `interoptopus_wire_create` (emitted by
//!    `builtins_wire!`) so that Rust allocates the buffer; the foreign side never allocates directly.
//! 2. **Serialize** — the value is serialized into that Rust-allocated buffer.
//! 3. **Transfer** — the `Wire<T>` is passed into an `#[ffi]` function. Rust receives ownership.
//! 4. **Deserialize** — [`Wire::unwire`] or [`Wire::try_unwire`] reads `T` from the buffer.
//! 5. **Free** — Rust drops the `Wire<T>` when the function returns, freeing the buffer.
//!
//!
//! ## Wire format
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
//! | `(A, B, …)` | Each element serialized in order |
//! | User structs | Each field serialized in declaration order |
//!
//! The wire format is not self-describing, both sides must agree on the exact type
//! layout.
//!
//! **Note:** This section describes an internal implementation detail that may change
//! between versions without notice. Do not rely on it for persistent storage or
//! cross-version compatibility.
mod buffer;

use crate::bad_wire;
use crate::inventory::{Inventory, TypeId};
use crate::lang::meta::{common_or_module_emission, Docs, Visibility};
use crate::lang::types::{SerializationError, Type, TypeInfo, TypeKind, TypePattern, WireIO};
use buffer::WireBuffer;
use std::marker::PhantomData;

/// Wraps and transfers complex objects over FFI.
///
/// The backing storage uses a (ptr, size) representation that can safely cross
/// FFI boundaries.
///
#[repr(C)]
pub struct Wire<T>
where
    T: ?Sized,
{
    buf: WireBuffer,
    _phantom: PhantomData<T>,
}

impl<T: TypeInfo + WireIO> Wire<T> {
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

impl<T: TypeInfo + WireIO> TypeInfo for Wire<T> {
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

impl<T: WireIO> WireIO for Wire<T> {
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

#[macro_export]
#[doc(hidden)]
macro_rules! __wire_create_body {
    ($size:ident, $out_len:ident, $out_capacity:ident) => {{
        if $size <= 0 {
            *$out_len = 0;
            *$out_capacity = 0;
            return ::std::ptr::null_mut();
        }
        let size = usize::try_from($size).expect("Invalid Wire buffer size");
        let mut vec: Vec<u8> = vec![0u8; size];
        let data = vec.as_mut_ptr();
        *$out_len = i32::try_from(vec.len()).expect("Too large Wire buffer");
        *$out_capacity = i32::try_from(vec.capacity()).expect("Too large Wire buffer");
        ::std::mem::forget(vec);
        data
    }};
}

/// Body of `interoptopus_wire_destroy`. Shared by [`builtins_wire!`] and
/// [`register_wire_trampolines!`].
#[macro_export]
#[doc(hidden)]
macro_rules! __wire_destroy_body {
    ($data:ident, $len:ident, $capacity:ident) => {{
        if $capacity <= 0 {
            return;
        }
        let _ = unsafe { Vec::from_raw_parts($data, usize::try_from($len).expect("Invalid vec length"), usize::try_from($capacity).expect("Invalid vec capacity")) };
    }};
}

/// Emits and registers helpers for [`Wire<T>`](crate::wire::Wire).
///
/// Backends (e.g., C#) use these functions internally so that foreign code can
/// allocate and free Rust-owned wire buffers.
///
/// # Usage
///
/// Call once in your inventory function and register the result:
///
/// ```rust
/// # use interoptopus::inventory::RustInventory;
/// # use interoptopus::builtins_wire;
/// pub fn inventory() -> RustInventory {
///     RustInventory::new()
///         .register(builtins_wire!())
///         // ... other registrations ...
///         .validate()
/// }
/// ```
///
/// # Implementation Details
///
/// This macro generates the following FFI functions:
/// - `interoptopus_wire_create` — allocates a wire buffer of a given size.
/// - `interoptopus_wire_destroy` — drops a wire buffer, freeing its memory.
/// Body of `interoptopus_wire_create`. Shared by [`builtins_wire!`] and
/// [`register_wire_trampolines!`].
#[macro_export]
macro_rules! builtins_wire {
    () => {{
        #[$crate::ffi(export = unique)]
        pub fn interoptopus_wire_create(size: i32, out_len: &mut i32, out_capacity: &mut i32) -> *mut u8 {
            $crate::__wire_create_body!(size, out_len, out_capacity)
        }

        #[$crate::ffi(export = unique)]
        pub fn interoptopus_wire_destroy(data: *mut u8, len: i32, capacity: i32) {
            $crate::__wire_destroy_body!(data, len, capacity)
        }

        |x: &mut $crate::inventory::RustInventory| {
            <interoptopus_wire_create as $crate::lang::function::FunctionInfo>::register(x);
            <interoptopus_wire_destroy as $crate::lang::function::FunctionInfo>::register(x);
        }
    }};
}

/// Registers wire buffer trampolines with a foreign plugin.
///
/// Defines local `extern "C"` functions (no exported symbols) that share
/// the same body logic as [`builtins_wire!`], then passes their pointers
/// to the given register callback.
///
/// # Example
///
/// ```rust,ignore
/// interoptopus::register_wire_trampolines!(|id, ptr| {
///     (plugin.register_trampoline)(id, ptr);
/// });
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! register_wire_trampolines {
    ($register_fn:expr) => {{
        extern "C" fn __wire_create(size: i32, out_len: &mut i32, out_capacity: &mut i32) -> *mut u8 {
            $crate::__wire_create_body!(size, out_len, out_capacity)
        }
        extern "C" fn __wire_destroy(data: *mut u8, len: i32, capacity: i32) {
            $crate::__wire_destroy_body!(data, len, capacity)
        }

        let __register: &mut dyn FnMut(i64, *const u8) = &mut $register_fn;
        __register($crate::trampoline::TRAMPOLINE_WIRE_CREATE, __wire_create as *const u8);
        __register($crate::trampoline::TRAMPOLINE_WIRE_DESTROY, __wire_destroy as *const u8);
    }};
}
