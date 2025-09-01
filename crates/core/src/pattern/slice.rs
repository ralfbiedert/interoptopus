//! Like a regular `&[T]` and `&mut [T]` but FFI safe.
//!
//! FFI slices work similar to Rust slices, except that they are FFI safe and map to "nice"
//! interop code. Internally they are represented by a pointer and length.
//!
//! # Example
//!
//! Here we export a function that wants the FFI equivalent of an `&[u32]`:
//!
//! ```
//! use interoptopus::{ffi_function};
//! use interoptopus::ffi;
//!
//! #[ffi_function]
//! pub fn call_with_slice(ffi_slice: ffi::Slice<u32>) {
//!     // ...
//! }
//! ```
//!
//! In backends supporting this pattern (e.g., C#), a function equivalent to the following
//! signature would be emitted, transparently handling all slice-related pinning and conversions:
//!
//! ```csharp
//! public static void call_with_slice(uint[] ffi_slice);
//! ```
//!
//! In C and unsupported backends the equivalent of this code will be emitted:
//!
//! ```c
//! typedef struct sliceu32
//!     {
//!     uint32_t* data;
//!     uint64_t len;
//!     } sliceu32;
//!
//! void call_with_slice(sliceu32 ffi_slice);
//! ```
//!

use crate::lang::util::capitalize_first_letter;
use crate::lang::{Composite, Docs, Field, Meta, Primitive, Representation, Type, Visibility};
use crate::lang::{Layout, TypeInfo};
use crate::pattern::TypePattern;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::{null, null_mut};

/// A representation of an array passed over an FFI boundary
#[repr(C)]
pub struct Slice<'a, T> {
    data: *const T,
    len: u64,
    _phantom: PhantomData<&'a T>,
}

impl<T> Copy for Slice<'_, T> {}

impl<T> Clone for Slice<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Default for Slice<'_, T> {
    fn default() -> Self {
        const {
            assert!(size_of::<Self>() == 16);
        }

        Self { data: null(), len: 0, _phantom: PhantomData }
    }
}

impl<'a, T> Slice<'a, T> {
    /// Create new Self from a normal slice.
    pub const fn from_slice(slice: &'a [T]) -> Self {
        Slice { data: slice.as_ptr(), len: slice.len() as u64, _phantom: PhantomData }
    }

    /// Returns a safe Rust slice.
    ///
    /// If the pointer was not null, the Rust slice will point to that data, otherwise an empty slice is returned.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn as_slice(&self) -> &'a [T] {
        if self.data.is_null() {
            &[]
        } else {
            // If non-null this should always point to valid data and the lifetime should be
            // guaranteed via the struct <'a>.
            unsafe { std::slice::from_raw_parts(self.data, self.len as usize) }
        }
    }
}

impl<'a, T> From<&'a [T]> for Slice<'a, T> {
    fn from(slice: &'a [T]) -> Self {
        Self::from_slice(slice)
    }
}

impl<T> Slice<'_, T>
where
    T: 'static,
{
    /// Creates a new empty slice.
    #[must_use]
    pub const fn empty() -> Self {
        let x: &'static [T] = &[];
        Self::from_slice(x)
    }
}

impl<T> Deref for Slice<'_, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

unsafe impl<T> TypeInfo for Slice<'_, T>
where
    T: TypeInfo,
{
    const RAW_SAFE: bool = T::RAW_SAFE;

    #[rustfmt::skip]
    fn type_info() -> Type {
        let doc_data = Docs::from_line("Pointer to start of immutable data.");
        let doc_len = Docs::from_line("Number of elements.");

        let fields = vec![
            Field::with_docs("data".to_string(), Type::ReadPointer(Box::new(T::type_info())), Visibility::Private, doc_data),
            Field::with_docs("len".to_string(), Type::Primitive(Primitive::U64), Visibility::Private, doc_len),
        ];

        let doc = Docs::from_line("A pointer to an array of data someone else owns which may not be modified.");
        let repr = Representation::new(Layout::C, None);
        let meta = Meta::with_module_docs(T::type_info().namespace().map_or_else(String::new, std::convert::Into::into), doc);
        let name = capitalize_first_letter(T::type_info().name_within_lib().as_str());
        let composite = Composite::with_meta_repr(format!("Slice{name}"), fields, meta, repr);
        let slice = SliceType::new(composite, T::type_info());
        Type::Pattern(TypePattern::Slice(slice))
    }
}

/// A representation of a mutable array passed over an FFI boundary
#[repr(C)]
pub struct SliceMut<'a, T> {
    data: *mut T,
    len: u64,
    _phantom: PhantomData<&'a mut T>,
}

impl<T> Default for SliceMut<'_, T> {
    fn default() -> Self {
        Self { data: null_mut(), len: 0, _phantom: PhantomData }
    }
}

impl<'a, T> SliceMut<'a, T> {
    /// Create new Self from a normal slice.
    pub fn from_slice(slice: &'a mut [T]) -> Self {
        SliceMut { data: slice.as_mut_ptr(), len: slice.len() as u64, _phantom: PhantomData::default() }
    }

    /// Returns a safe, mutable Rust slice.
    ///
    /// If the pointer was not null, the Rust slice will point to that data, otherwise an empty slice is returned.
    #[allow(clippy::cast_possible_truncation)]
    pub fn as_slice_mut(&mut self) -> &'a mut [T] {
        if self.data.is_null() {
            &mut []
        } else {
            // If non-null this should always point to valid data and the lifetime should be
            // guaranteed via the struct <'a>.
            unsafe { std::slice::from_raw_parts_mut(self.data, self.len as usize) }
        }
    }

    /// Returns a safe Rust slice.
    ///
    /// If the pointer was not null, the Rust slice will point to that data, otherwise an empty slice is returned.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn as_slice(&self) -> &'a [T] {
        if self.data.is_null() {
            &[]
        } else {
            // If non-null this should always point to valid data and the lifetime should be
            // guaranteed via the struct <'a>.
            unsafe { std::slice::from_raw_parts(self.data, self.len as usize) }
        }
    }
}

impl<T> SliceMut<'_, T>
where
    T: 'static,
{
    /// Creates a new empty slice.
    #[must_use]
    pub fn empty() -> Self {
        let x: &'static mut [T] = &mut [];
        Self::from_slice(x)
    }
}

impl<'a, T> From<&'a mut [T]> for SliceMut<'a, T> {
    fn from(slice: &'a mut [T]) -> Self {
        Self::from_slice(slice)
    }
}

impl<T> Deref for SliceMut<'_, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T> DerefMut for SliceMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_slice_mut()
    }
}

unsafe impl<T> TypeInfo for SliceMut<'_, T>
where
    T: TypeInfo,
{
    const RAW_SAFE: bool = T::RAW_SAFE;

    #[rustfmt::skip]
    fn type_info() -> Type {
        let doc_data = Docs::from_line("Pointer to start of mutable data.");
        let doc_len = Docs::from_line("Number of elements.");
        let fields = vec![
            Field::with_docs("data".to_string(), Type::ReadPointer(Box::new(T::type_info())), Visibility::Private, doc_data),
            Field::with_docs("len".to_string(), Type::Primitive(Primitive::U64), Visibility::Private, doc_len),
        ];

        let doc = Docs::from_line("A pointer to an array of data someone else owns which may be modified.");
        let repr = Representation::new(Layout::C, None);
        let meta = Meta::with_module_docs(T::type_info().namespace().map_or_else(String::new, std::convert::Into::into), doc);
        let name = capitalize_first_letter(T::type_info().name_within_lib().as_str());
        let composite = Composite::with_meta_repr(format!("SliceMut{name}"), fields, meta, repr);
        let slice = SliceType::new(composite, T::type_info());
        Type::Pattern(TypePattern::SliceMut(slice))
    }
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SliceType {
    composite_type: Composite,
    target_type: Box<Type>,
}

impl SliceType {
    #[must_use]
    pub fn new(composite_type: Composite, target_type: Type) -> Self {
        Self { composite_type, target_type: Box::new(target_type) }
    }

    #[must_use]
    pub fn rust_name(&self) -> &str {
        self.composite_type.rust_name()
    }

    #[must_use]
    pub fn meta(&self) -> &Meta {
        self.composite_type.meta()
    }

    #[must_use]
    pub fn composite_type(&self) -> &Composite {
        &self.composite_type
    }

    #[must_use]
    pub fn t(&self) -> &Type {
        &self.target_type
    }

    #[must_use]
    pub fn to_type(&self) -> Type {
        Type::Pattern(TypePattern::Slice(self.clone()))
    }
}

#[cfg(test)]
mod test {
    use crate::pattern::slice::{Slice, SliceMut};

    #[test]
    fn can_create_ref() {
        let slice = &[0, 1, 2, 3, 5];
        let empty = Slice::<u8>::empty();
        let some = Slice::<u8>::from_slice(slice);

        assert_eq!(empty.as_slice(), &[] as &[u8]);
        assert_eq!(some.as_slice(), slice);
    }

    #[test]
    fn can_create_mut() {
        let slice = &mut [0, 1, 2, 3, 5];
        let empty = SliceMut::<u8>::empty();
        let mut some = SliceMut::<u8>::from_slice(slice.as_mut());
        let sub = &mut some[1..=2];

        sub[0] = 6;
        some[0] = 5;

        assert_eq!(empty.as_slice(), &[] as &[u8]);
        assert_eq!(slice, &[5, 6, 2, 3, 5]);
    }

    #[test]
    fn multi_borrow_mut_slice() {
        let slice = &mut [0, 1, 2, 3, 5];
        let empty = SliceMut::<u8>::empty();
        let target: &mut [u8] = {
            let mut some = SliceMut::<u8>::from_slice(slice.as_mut());
            some.as_slice_mut()
        };
        let sub = &mut target[1..=2];

        sub[0] = 6;
        target[0] = 5;

        assert_eq!(empty.as_slice(), &[] as &[u8]);
        assert_eq!(slice, &[5, 6, 2, 3, 5]);
    }
}
