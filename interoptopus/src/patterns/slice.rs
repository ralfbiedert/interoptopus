//! Like a regular `&[T]` but FFI safe.

use crate::lang::c::{CType, CompositeType, Field, PrimitiveType};
use crate::lang::rust::CTypeInfo;
use std::marker::PhantomData;

/// A representation of an array passed over an FFI boundary
#[repr(C)]
pub struct FFISlice<'a, T> {
    slice_ptr: *const T,
    len: u64,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> FFISlice<'a, T> {
    pub fn from_slice(slice: &[T]) -> Self {
        FFISlice {
            slice_ptr: slice.as_ptr(),
            len: slice.len() as u64,
            _phantom: Default::default(),
        }
    }

    pub fn as_slice(&'a self) -> Option<&'a [T]> {
        if self.slice_ptr.is_null() {
            None
        } else {
            // If non-null this should always point to valid data and the lifetime should be
            // guaranteed via the struct <'a>.
            Some(unsafe { std::slice::from_raw_parts(self.slice_ptr, self.len as usize) })
        }
    }
}

impl<'a, T> CTypeInfo for FFISlice<'a, T>
where
    T: CTypeInfo,
{
    fn type_info() -> CType {
        let mut composite = CompositeType::new(format!("FFISlice{}", T::type_info().name_within_lib()));
        composite.add_field(Field::new("slice_ptr".to_string(), CType::ReadPointer(Box::new(T::type_info()))));
        composite.add_field(Field::new("len".to_string(), CType::Primitive(PrimitiveType::U64)));
        CType::Composite(composite)
    }
}
