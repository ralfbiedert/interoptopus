//! Like a regular [`Option`] but FFI safe.
use crate::lang::c::{CType, CompositeType, Field, PrimitiveType};
use crate::lang::rust::CTypeInfo;

/// An option-like type at the FFI boundary where a regular [`Option`] doesn't work.
#[repr(C)]
pub struct FFIOption<T> {
    t: T,
    is_some: u8,
}

impl<T> CTypeInfo for FFIOption<T>
where
    T: CTypeInfo,
{
    fn type_info() -> CType {
        let mut composite = CompositeType::new(format!("FFIOption{}", T::type_info().name_within_lib()));
        composite.add_field(Field::new("t".to_string(), T::type_info()));
        composite.add_field(Field::new("is_some".to_string(), CType::Primitive(PrimitiveType::U8)));
        CType::Composite(composite)
    }
}
