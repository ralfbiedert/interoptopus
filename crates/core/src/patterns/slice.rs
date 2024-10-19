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
//! use interoptopus::patterns::slice::FFISlice;
//!
//! #[ffi_function]
//! #[no_mangle]
//! pub extern "C" fn call_with_slice(ffi_slice: FFISlice<u32>) {
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

use crate::lang::c::{CType, CompositeType, Documentation, Field, Layout, Meta, PrimitiveType, Representation, Visibility};
use crate::lang::rust::CTypeInfo;
use crate::patterns::TypePattern;
use crate::util::capitalize_first_letter;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::{null, null_mut};

/// A representation of an array passed over an FFI boundary
#[repr(C)]
pub struct FFISlice<'a, T> {
    data: *const T,
    len: u64,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> Copy for FFISlice<'a, T> {}

impl<'a, T> Clone for FFISlice<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T> Default for FFISlice<'a, T> {
    fn default() -> Self {
        Self {
            data: null(),
            len: 0,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T> FFISlice<'a, T> {
    /// Create new Self from a normal slice.
    pub fn from_slice(slice: &'a [T]) -> Self {
        FFISlice {
            data: slice.as_ptr(),
            len: slice.len() as u64,
            _phantom: Default::default(),
        }
    }

    /// Returns a safe Rust slice.
    ///
    /// If the pointer was not null, the Rust slice will point to that data, otherwise an empty slice is returned.
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

impl<'a, T> From<&'a [T]> for FFISlice<'a, T> {
    fn from(slice: &'a [T]) -> Self {
        Self::from_slice(slice)
    }
}

impl<'a, T> FFISlice<'a, T>
where
    T: 'static,
{
    /// Creates a new empty slice.
    pub fn empty() -> Self {
        let x: &'static [T] = &[];
        Self::from_slice(x)
    }
}

impl<'a, T> Deref for FFISlice<'a, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

unsafe impl<'a, T> CTypeInfo for FFISlice<'a, T>
where
    T: CTypeInfo,
{
    #[rustfmt::skip]
    fn type_info() -> CType {
        let doc_data = Documentation::from_line("Pointer to start of immutable data.");
        let doc_len = Documentation::from_line("Number of elements.");

        let fields = vec![
            Field::with_documentation("data".to_string(), CType::ReadPointer(Box::new(T::type_info())), Visibility::Private, doc_data),
            Field::with_documentation("len".to_string(), CType::Primitive(PrimitiveType::U64), Visibility::Private, doc_len),
        ];

        let doc = Documentation::from_line("A pointer to an array of data someone else owns which may not be modified.");
        let repr = Representation::new(Layout::C, None);
        let meta = Meta::with_namespace_documentation(T::type_info().namespace().map(|e| e.into()).unwrap_or_else(String::new), doc);
        let name = capitalize_first_letter(T::type_info().name_within_lib());
        let composite = CompositeType::with_meta_repr(format!("Slice{}", name), fields, meta, repr);
        CType::Pattern(TypePattern::Slice(composite))
    }
}

/// A representation of a mutable array passed over an FFI boundary
#[repr(C)]
pub struct FFISliceMut<'a, T> {
    data: *mut T,
    len: u64,
    _phantom: PhantomData<&'a mut T>,
}

impl<'a, T> Default for FFISliceMut<'a, T> {
    fn default() -> Self {
        Self {
            data: null_mut(),
            len: 0,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T> FFISliceMut<'a, T> {
    /// Create new Self from a normal slice.
    pub fn from_slice(slice: &'a mut [T]) -> Self {
        FFISliceMut {
            data: slice.as_mut_ptr(),
            len: slice.len() as u64,
            _phantom: Default::default(),
        }
    }

    /// Returns a safe, mutable Rust slice.
    ///
    /// If the pointer was not null, the Rust slice will point to that data, otherwise an empty slice is returned.
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

impl<'a, T> FFISliceMut<'a, T>
where
    T: 'static,
{
    /// Creates a new empty slice.
    pub fn empty() -> Self {
        let x: &'static mut [T] = &mut [];
        Self::from_slice(x)
    }
}

impl<'a, T> From<&'a mut [T]> for FFISliceMut<'a, T> {
    fn from(slice: &'a mut [T]) -> Self {
        Self::from_slice(slice)
    }
}

impl<'a, T> Deref for FFISliceMut<'a, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<'a, T> DerefMut for FFISliceMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_slice_mut()
    }
}

unsafe impl<'a, T> CTypeInfo for FFISliceMut<'a, T>
where
    T: CTypeInfo,
{
    #[rustfmt::skip]
    fn type_info() -> CType {
        let doc_data = Documentation::from_line("Pointer to start of mutable data.");
        let doc_len = Documentation::from_line("Number of elements.");
        let fields = vec![
            Field::with_documentation("data".to_string(), CType::ReadPointer(Box::new(T::type_info())), Visibility::Private, doc_data),
            Field::with_documentation("len".to_string(), CType::Primitive(PrimitiveType::U64), Visibility::Private, doc_len),
        ];

        let doc = Documentation::from_line("A pointer to an array of data someone else owns which may be modified.");
        let repr = Representation::new(Layout::C, None);
        let meta = Meta::with_namespace_documentation(T::type_info().namespace().map(|e| e.into()).unwrap_or_else(String::new), doc);
        let name = capitalize_first_letter(T::type_info().name_within_lib());
        let composite = CompositeType::with_meta_repr(format!("SliceMut{}", name), fields, meta, repr);
        CType::Pattern(TypePattern::SliceMut(composite))
    }
}

#[cfg(test)]
mod test {
    use crate::patterns::slice::{FFISlice, FFISliceMut};

    #[test]
    fn can_create_ref() {
        let slice = &[0, 1, 2, 3, 5];
        let empty = FFISlice::<u8>::empty();
        let some = FFISlice::<u8>::from_slice(slice);

        assert_eq!(empty.as_slice(), &[]);
        assert_eq!(some.as_slice(), slice);
    }

    #[test]
    fn can_create_mut() {
        let slice = &mut [0, 1, 2, 3, 5];
        let empty = FFISliceMut::<u8>::empty();
        let mut some = FFISliceMut::<u8>::from_slice(slice.as_mut());
        let sub = &mut some[1..=2];

        sub[0] = 6;
        some[0] = 5;

        assert_eq!(empty.as_slice(), &[]);
        assert_eq!(slice, &[5, 6, 2, 3, 5]);
    }

    #[test]
    fn multi_borrow_mut_slice() {
        let slice = &mut [0, 1, 2, 3, 5];
        let empty = FFISliceMut::<u8>::empty();
        let target: &mut [u8] = {
            let mut some = FFISliceMut::<u8>::from_slice(slice.as_mut());
            some.as_slice_mut()
        };
        let sub = &mut target[1..=2];

        sub[0] = 6;
        target[0] = 5;

        assert_eq!(empty.as_slice(), &[]);
        assert_eq!(slice, &[5, 6, 2, 3, 5]);
    }
}
