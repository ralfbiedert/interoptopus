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
        let mut fields = Vec::new();
        fields.push(Field::new("t".to_string(), T::type_info()));
        fields.push(Field::new("is_some".to_string(), CType::Primitive(PrimitiveType::U8)));

        let composite = CompositeType::new(format!("FFIOption{}", T::type_info().name_within_lib()), fields);
        CType::Composite(composite)
    }
}
