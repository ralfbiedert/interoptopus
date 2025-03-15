//! A conversion helper when you need to emit interop for types you don't own.
//!
//! # Example
//!
//! Let's imagine you rely on `game_engine::Vec3` that comes from a foreign crate and
//! you can't attach `#[ffi_type]` to it. Instead you can define your own type `LocalVec3`
//! and use it as `Surrogate<Vec3, LocalVec3>` in your interfaces. That way you will
//! get zero-cost conversion helpers for free.
//!
//! ```
//! use interoptopus::{ffi_function, ffi_type};
//! use interoptopus::pattern::surrogate::{CorrectSurrogate, Surrogate};
//! #
//! # mod foreign {
//! #    #[repr(C)]
//! #    pub struct Vec3 {
//! #         x: f32,
//! #         y: f32,
//! #         z: f32,
//! #    }
//! # }
//!
//! // Create a LocalVec3 with matching fields to your upstream type.
//! #[ffi_type]
//! pub struct LocalVec3 {
//!     x: f32,
//!     y: f32,
//!     z: f32,
//! }
//!
//! // This marker trait guarantees `LocalVec3` is a valid surrogate
//! // for `Vec3`. You must ensure this is correct, or you get UB.
//! unsafe impl CorrectSurrogate<foreign::Vec3> for LocalVec3 {}
//!
//! // And here we create a nicer alias.
//! type Vec3 = Surrogate<foreign::Vec3, LocalVec3>;
//!
//! #[ffi_function]
//! pub fn do_compute(s: Vec3) {
//!     let vec: foreign::Vec3 = s.into_t();
//! }
//! ```
//!
//! # Usage Note
//!
//! Surrogates are a niche feature to save you some implementation overhead in certain situations.
//! In most cases the right things to do is defining your own FFI types and export these instead.

use crate::lang::Type;
use crate::lang::TypeInfo;
use std::marker::PhantomData;
use std::mem::{ManuallyDrop, transmute};

/// A marker trait for types that are surrogates for other types.
///
/// # Safety
///
/// You must ensure the types match, otherwise undefined behavior will occur. In particular you
/// must ensure that:
///
/// - Both types `T` and `L` have the same fields in the same order.
/// - Both types are `repr(C)` and otherwise agree in alignment, layout and size.
pub unsafe trait CorrectSurrogate<T> {}

/// A type mapper at the FFI boundary.
#[repr(transparent)]
pub struct Surrogate<T, L: TypeInfo> {
    inner: T,
    _marker: PhantomData<L>,
}

unsafe impl<T, L: TypeInfo + CorrectSurrogate<T>> TypeInfo for Surrogate<T, L> {
    fn type_info() -> Type {
        assert_eq!(size_of::<T>(), size_of::<L>());
        L::type_info()
    }
}

impl<T, L: TypeInfo + CorrectSurrogate<T>> Surrogate<T, L> {
    /// Creates a new `Surrogate` from a `T`.
    pub const fn from_t(x: T) -> Self {
        Self { inner: x, _marker: PhantomData }
    }

    /// Creates a new `Surrogate` from a `L`.
    pub fn from_l(x: L) -> Self {
        let t = unsafe {
            let this = ManuallyDrop::new(x);
            std::ptr::read(std::ptr::from_ref::<L>(&*this).cast::<T>())
        };

        Self { inner: t, _marker: PhantomData::default() }
    }

    /// Views the type as a `T`.
    pub const fn as_t(&self) -> &T {
        &self.inner
    }

    /// Views the type mutably as a `T`.
    pub fn as_t_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Views the type as a `L`.
    #[allow(clippy::transmute_ptr_to_ptr)]
    pub fn as_l(&self) -> &L {
        // Safety: this should be guaranteed through the `CorrectSurrogate` trait.
        unsafe { transmute(&self.inner) }
    }

    /// Views the type mutably as a `L`.
    #[allow(clippy::transmute_ptr_to_ptr)]
    pub fn as_l_mut(&mut self) -> &mut L {
        // Safety: this should be guaranteed through the `CorrectSurrogate` trait.
        unsafe { transmute(&mut self.inner) }
    }

    /// Converts the type into an `T`.
    pub fn into_t(self) -> T {
        self.inner
    }

    /// Converts the type into an `L`.
    pub fn into_l(self) -> L {
        // Safety: this should be guaranteed through the `CorrectSurrogate` trait.
        unsafe {
            let this = ManuallyDrop::new(self);
            std::ptr::read(std::ptr::from_ref::<T>(&this.inner).cast::<L>())
        }
    }
}
