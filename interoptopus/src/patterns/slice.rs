//! Like a regular `&[T]` and `&mut [T]` but FFI safe.

use crate::lang::c::{CType, CompositeType, Documentation, Field, PrimitiveType, Visibility};
use crate::lang::rust::CTypeInfo;
use crate::patterns::TypePattern;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

/// A representation of an array passed over an FFI boundary
#[repr(C)]
pub struct FFISlice<'a, T> {
    data: *const T,
    len: u64,
    _phantom: PhantomData<&'a T>,
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

    /// Tries to return a slice if the pointer was not null.
    pub fn as_slice<'b>(&'b self) -> Option<&'b [T]>
    where
        'a: 'b,
    {
        if self.data.is_null() {
            None
        } else {
            // If non-null this should always point to valid data and the lifetime should be
            // guaranteed via the struct <'a>.
            Some(unsafe { std::slice::from_raw_parts(self.data, self.len as usize) })
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
        self.as_slice().unwrap_or(&[])
    }
}

unsafe impl<'a, T> CTypeInfo for FFISlice<'a, T>
where
    T: CTypeInfo,
{
    #[rustfmt::skip]
    fn type_info() -> CType {
        let fields = vec![
            Field::with_documentation("data".to_string(), CType::ReadPointer(Box::new(T::type_info())), Visibility::Private, Documentation::new()),
            Field::with_documentation("len".to_string(), CType::Primitive(PrimitiveType::U64), Visibility::Private, Documentation::new()),
        ];

        let composite = CompositeType::new(format!("FFISlice{}", T::type_info().name_within_lib()), fields);
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

impl<'a, T> FFISliceMut<'a, T> {
    /// Create new Self from a normal slice.
    pub fn from_slice(slice: &'a mut [T]) -> Self {
        FFISliceMut {
            data: slice.as_mut_ptr(),
            len: slice.len() as u64,
            _phantom: Default::default(),
        }
    }

    /// Tries to return a slice if the pointer was not null.
    pub fn as_slice_mut<'b>(&'b mut self) -> Option<&'b mut [T]>
    where
        'a: 'b,
    {
        if self.data.is_null() {
            None
        } else {
            // If non-null this should always point to valid data and the lifetime should be
            // guaranteed via the struct <'a>.
            Some(unsafe { std::slice::from_raw_parts_mut(self.data, self.len as usize) })
        }
    }

    /// Tries to return a slice if the pointer was not null.
    pub fn as_slice<'b>(&'b self) -> Option<&'b [T]>
    where
        'a: 'b,
    {
        if self.data.is_null() {
            None
        } else {
            // If non-null this should always point to valid data and the lifetime should be
            // guaranteed via the struct <'a>.
            Some(unsafe { std::slice::from_raw_parts(self.data, self.len as usize) })
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

impl<'a, T> Deref for FFISliceMut<'a, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice().unwrap_or(&[])
    }
}

impl<'a, T> DerefMut for FFISliceMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_slice_mut().unwrap_or(&mut [])
    }
}

unsafe impl<'a, T> CTypeInfo for FFISliceMut<'a, T>
where
    T: CTypeInfo,
{
    #[rustfmt::skip]
    fn type_info() -> CType {
        let fields = vec![
            Field::with_documentation("data".to_string(), CType::ReadPointer(Box::new(T::type_info())), Visibility::Private, Documentation::new()),
            Field::with_documentation("len".to_string(), CType::Primitive(PrimitiveType::U64), Visibility::Private, Documentation::new()),
        ];

        let composite = CompositeType::new(format!("FFISliceMut{}", T::type_info().name_within_lib()), fields);
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

        assert_eq!(empty.as_slice().unwrap(), &[]);
        assert_eq!(some.as_slice().unwrap(), slice);
    }

    #[test]
    fn can_create_mut() {
        let slice = &mut [0, 1, 2, 3, 5];
        let empty = FFISliceMut::<u8>::empty();
        let mut some = FFISliceMut::<u8>::from_slice(slice.as_mut());
        let sub = &mut some[1..=2];

        sub[0] = 6;
        some[0] = 5;

        assert_eq!(empty.as_slice().unwrap(), &[]);
        assert_eq!(slice, &[5, 6, 2, 3, 5]);
    }
}
