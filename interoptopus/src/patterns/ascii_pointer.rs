use crate::lang::c::CType;
use crate::lang::rust::CTypeInfo;
use crate::patterns::TypePattern;
use crate::Error;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::option::Option::None;
use std::os::raw::c_char;
use std::ptr::null;

/// Represents a `*const char` on FFI level pointing to an `0x0` terminated ASCII string.
#[repr(transparent)]
pub struct AsciiPointer0In<'a> {
    ptr: *const c_char,
    _phandom: PhantomData<&'a ()>,
}

impl<'a> Default for AsciiPointer0In<'a> {
    fn default() -> Self {
        Self {
            ptr: null(),
            _phandom: Default::default(),
        }
    }
}

impl<'a> AsciiPointer0In<'a> {
    /// Create a new `null` ascii pointer.
    pub fn null() -> Self {
        Self::default()
    }

    /// Create a [`CStr`] for the pointer.
    pub fn as_c_str(&self) -> Option<&'a CStr> {
        if self.ptr.is_null() {
            None
        } else {
            // TODO: Write something about safety
            unsafe { Some(CStr::from_ptr(self.ptr)) }
        }
    }

    pub fn as_str(&self) -> Result<&'a str, Error> {
        Ok(self.as_c_str().ok_or(Error::Null)?.to_str()?)
    }
}

impl<'a> CTypeInfo for AsciiPointer0In<'a> {
    fn type_info() -> CType {
        CType::Pattern(TypePattern::AsciiPointer)
    }
}
