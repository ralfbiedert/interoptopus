//! Like a regular `&[T]` but FFI safe.

use crate::lang::c::{CType, CompositeType, Documentation, Field, PrimitiveType, Visibility};
use crate::lang::rust::CTypeInfo;
use crate::patterns::TypePattern;
use std::marker::PhantomData;
use std::ops::Deref;

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
    pub fn as_slice(&'a self) -> Option<&'a [T]> {
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

#[cfg(test)]
mod test {
    use crate::patterns::slice::FFISlice;

    #[test]
    fn can_create() {
        let slice = &[0, 1, 2, 3, 5];
        let empty = FFISlice::<u8>::empty();
        let some = FFISlice::<u8>::from_slice(slice);

        assert_eq!(empty.as_slice().unwrap(), &[]);
        assert_eq!(some.as_slice().unwrap(), slice);
    }
}
