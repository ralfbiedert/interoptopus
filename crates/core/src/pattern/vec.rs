//! Owned, FFI-safe growable array.
//!
//! [`Vec<T>`] is a `repr(C)` type that mirrors the layout of a Rust
//! `std::vec::Vec<T>` (pointer + length + capacity). It owns its
//! allocation and can be passed by value across the FFI boundary.
//! Backends generate idiomatic collection wrappers — for example, in C#
//! each `Vec<T>` becomes a typed class with indexer, enumerator, and
//! disposal logic.
//!
//! The [`builtins_vec!`](crate::builtins_vec) macro must be registered
//! in the inventory for each element type so that backends can emit the
//! required create / destroy helper functions.
//!
//! # Example
//!
//! ```
//! use interoptopus::ffi;
//!
//! #[ffi]
//! pub fn sum(values: ffi::Vec<f64>) -> f64 {
//!     values.into_vec().iter().sum()
//! }
//! ```

use crate::inventory::{Inventory, TypeId};
use crate::lang::meta::{common_or_module_emission, Docs, Visibility};
use crate::lang::types::{Type, TypeInfo, TypeKind, TypePattern, WireIO};
use crate::wire::SerializationError;
use std::io::{Read, Write};
use std::mem::forget;

/// Owned, FFI-safe growable array with the same layout as `std::vec::Vec<T>`.
///
/// The type is `repr(C)` with fields `(ptr, len, capacity)` using `u64`
/// lengths for stable cross-platform ABI. Ownership transfers across the
/// FFI boundary: once a `Vec<T>` is handed to foreign code, that side is
/// responsible for calling the generated destroy helper.
///
/// Use [`from_vec`](Self::from_vec) or the `From<std::vec::Vec<T>>` impl
/// to create, and [`into_vec`](Self::into_vec) to consume back into a
/// standard `Vec`.
#[derive(Debug)]
#[repr(C)]
pub struct Vec<T> {
    ptr: *mut T,
    len: u64,
    capacity: u64,
}

unsafe impl<T> Send for Vec<T> where T: Send {}
unsafe impl<T> Sync for Vec<T> where T: Sync {}

impl<T> Vec<T> {
    #[must_use]
    pub fn from_vec(mut s: std::vec::Vec<T>) -> Self {
        let ptr = s.as_mut_ptr();
        let capacity = s.capacity() as u64;
        let len = s.len() as u64;
        forget(s);
        Self { ptr, len, capacity }
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn len(&self) -> usize {
        self.len as usize
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn into_vec(self) -> std::vec::Vec<T> {
        let rval = unsafe { std::vec::Vec::from_raw_parts(self.ptr, self.len as usize, self.capacity as usize) };
        forget(self);
        rval
    }
}

impl<T: Clone + TypeInfo> Clone for Vec<T> {
    #[allow(clippy::cast_possible_truncation)]
    fn clone(&self) -> Self {
        let this = unsafe { std::vec::Vec::from_raw_parts(self.ptr, self.len as usize, self.capacity as usize) };
        let rval = this.clone();
        forget(this);
        rval.into()
    }
}

impl<T: TypeInfo> From<std::vec::Vec<T>> for Vec<T> {
    fn from(value: std::vec::Vec<T>) -> Self {
        Self::from_vec(value)
    }
}

impl<T: TypeInfo> From<Vec<T>> for std::vec::Vec<T> {
    fn from(value: Vec<T>) -> Self {
        value.into_vec()
    }
}

unsafe impl<T: TypeInfo> TypeInfo for Vec<T> {
    const WIRE_SAFE: bool = false;
    const RAW_SAFE: bool = true;
    const ASYNC_SAFE: bool = true;
    const SERVICE_SAFE: bool = false;
    const SERVICE_CTOR_SAFE: bool = false;

    fn id() -> TypeId {
        TypeId::new(0x8046F09DA6F9059E4A0D8327DDE7FAC8).derive_id(T::id())
    }

    fn kind() -> TypeKind {
        TypeKind::TypePattern(TypePattern::Vec(T::id()))
    }

    fn ty() -> Type {
        let t = T::ty();
        Type {
            name: format!("Vec<{}>", t.name),
            visibility: Visibility::Public,
            docs: Docs::default(),
            emission: common_or_module_emission(&[t.emission]),
            kind: Self::kind(),
        }
    }

    fn register(inventory: &mut impl Inventory) {
        T::register(inventory);
        inventory.register_type(Self::id(), Self::ty());
    }
}

unsafe impl<T: WireIO> WireIO for Vec<T> {
    fn write(&self, _: &mut impl Write) -> Result<(), SerializationError> {
        todo!()
    }

    fn read(_: &mut impl Read) -> Result<Self, SerializationError> {
        todo!()
    }

    fn live_size(&self) -> usize {
        todo!()
    }
}

impl<T> Drop for Vec<T> {
    #[allow(clippy::cast_possible_truncation)]
    fn drop(&mut self) {
        if self.ptr.is_null() {
            return;
        }
        unsafe {
            let _ = std::vec::Vec::from_raw_parts(self.ptr, self.len as usize, self.capacity as usize);
        }
    }
}

/// Emits and registers helpers for [`ffi::Vec<T>`](crate::pattern::vec::Vec) types.
///
/// Unlike [`builtins_string!`](crate::builtins_string), this macro must be invoked **once
/// per element type** you want to pass across the FFI boundary, because each concrete `Vec<T>`
/// requires its own set of helper functions.
///
/// # Usage
///
/// ```rust
/// # use interoptopus::inventory::RustInventory;
/// # use interoptopus::{ffi, builtins_vec};
/// # #[derive(Copy, Clone, Debug)]
/// # #[repr(C)]
/// # pub struct Vec3f32 { pub x: f32, pub y: f32, pub z: f32 }
/// # unsafe impl interoptopus::lang::types::TypeInfo for Vec3f32 {
/// #     const WIRE_SAFE: bool = false; const RAW_SAFE: bool = true; const ASYNC_SAFE: bool = true;
/// #     const SERVICE_SAFE: bool = false; const SERVICE_CTOR_SAFE: bool = false;
/// #     fn id() -> interoptopus::inventory::TypeId { interoptopus::inventory::TypeId::new(0) }
/// #     fn kind() -> interoptopus::lang::types::TypeKind { todo!() }
/// #     fn ty() -> interoptopus::lang::types::Type { todo!() }
/// #     fn register(_: &mut impl interoptopus::inventory::Inventory) {}
/// # }
/// pub fn inventory() -> RustInventory {
///     RustInventory::new()
///         .register(builtins_vec!(u8))
///         .register(builtins_vec!(ffi::String))
///         .register(builtins_vec!(Vec3f32))
///         // ... other registrations ...
///         .validate()
/// }
/// ```
///
/// # Implementation Details
///
/// This macro generates the following FFI functions with unique, type-specific symbol names:
/// - `interoptopus_vec_create` — creates an `ffi::Vec<T>` by copying `len` elements from a raw pointer.
/// - `interoptopus_vec_destroy` — drops an `ffi::Vec<T>`, freeing its memory.
///
#[macro_export]
macro_rules! builtins_vec {
    ($t:ty) => {{
        #[$crate::ffi(export = unique)]
        pub fn interoptopus_vec_create(data: *const ::std::ffi::c_void, len: u64, rval: &mut ::std::mem::MaybeUninit<$crate::pattern::vec::Vec<$t>>) -> i64 {
            let slice = if data.is_null() {
                &[]
            } else {
                unsafe { ::std::slice::from_raw_parts::<$t>(data.cast(), len as usize) }
            };
            let vec = slice.to_vec();
            rval.write($crate::pattern::vec::Vec::from_vec(vec));
            0
        }

        #[$crate::ffi(export = unique)]
        pub fn interoptopus_vec_destroy(_: $crate::ffi::Vec<$t>) -> i64 {
            0
        }

        |x: &mut $crate::inventory::RustInventory| {
            <interoptopus_vec_create as $crate::lang::function::FunctionInfo>::register(x);
            <interoptopus_vec_destroy as $crate::lang::function::FunctionInfo>::register(x);
        }
    }};
}
