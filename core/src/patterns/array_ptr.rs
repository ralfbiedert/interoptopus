use crate::lang::c::CType;
use crate::lang::rust::CTypeInfo;
use crate::patterns::TypePattern;
use std::marker::PhantomData;

#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize))]
#[cfg_attr(not(feature = "serde"), derive(Debug, Copy, Clone, PartialEq))]
pub struct ArrayPointer<'a, T> {
    data: *const T,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> Default for ArrayPointer<'a, T> {
    fn default() -> Self {
        ArrayPointer {
            data: std::ptr::null(),
            _phantom: PhantomData::default(),
        }
    }
}

unsafe impl<'a, T> CTypeInfo for ArrayPointer<'a, T>
where
    T: CTypeInfo,
{
    #[rustfmt::skip]
    fn type_info() -> CType {
        CType::Pattern(TypePattern::ArrayPointer(Box::new(T::type_info())))
    }
}

impl<'a, T> ArrayPointer<'a, T> {
    pub unsafe fn as_slice<'b>(&'b self, len: u64) -> &'b [T]
    where
        'a: 'b,
    {
        if self.data.is_null() {
            &[]
        } else {
            // Dependent on user passing in the correct length, hence function marked as unsafe
            std::slice::from_raw_parts(self.data, len as usize)
        }
    }
}
