//! Raw `*const char` pointer on C-level but ASCII `string` like in languages that support it.

use crate::lang::c::CType;
use crate::lang::rust::CTypeInfo;
use crate::patterns::TypePattern;
use crate::Error;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::option::Option::None;
use std::os::raw::c_char;
use std::ptr::null;

static EMPTY: &[u8] = b"\0";

/// Represents a `*const char` on FFI level pointing to an `0x0` terminated ASCII string.
#[repr(transparent)]
#[derive(Debug)]
pub struct AsciiPointer<'a> {
    ptr: *const c_char,
    _phandom: PhantomData<&'a ()>,
}

impl<'a> Default for AsciiPointer<'a> {
    fn default() -> Self {
        Self {
            ptr: null(),
            _phandom: Default::default(),
        }
    }
}

impl<'a> AsciiPointer<'a> {
    pub fn empty() -> Self {
        Self {
            ptr: EMPTY.as_ptr().cast(),
            _phandom: Default::default(),
        }
    }

    /// Create a pointer from a CStr.
    pub fn from_cstr(cstr: &'a CStr) -> Self {
        Self {
            ptr: cstr.as_ptr(),
            _phandom: Default::default(),
        }
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

    /// Attempts to return a Rust `str`.
    pub fn as_str(&self) -> Result<&'a str, Error> {
        Ok(self.as_c_str().ok_or(Error::Null)?.to_str()?)
    }
}

unsafe impl<'a> CTypeInfo for AsciiPointer<'a> {
    fn type_info() -> CType {
        CType::Pattern(TypePattern::AsciiPointer)
    }
}

#[cfg(test)]
mod test {
    use crate::patterns::ascii_pointer::AsciiPointer;
    use std::ffi::CString;

    #[test]
    fn can_create() {
        let s = "hello world";
        let cstr = CString::new(s).unwrap();

        let ptr_some = AsciiPointer::from_cstr(&cstr);

        assert_eq!(s, ptr_some.as_str().unwrap());
    }
}
