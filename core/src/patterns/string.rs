//! Raw `*const char` pointer on C-level but ASCII `string` like in languages that support it.
//!
//! # Example
//!
//! In your library you can accept ASCII strings like this:
//!
//! ```
//! use interoptopus::ffi_function;
//! use interoptopus::patterns::string::AsciiPointer;
//!
//! #[ffi_function]
//! #[no_mangle]
//! pub extern "C" fn call_with_string(s: AsciiPointer)  {
//!     //
//! # s.as_str().unwrap();
//! }
//! ```
//!
//! Backends supporting this pattern might generate the equivalent to the following pseudo-code:
//!
//! ```csharp
//! void call_with_string(string s);
//! ```
//!
//! Backends not supporting this pattern, and C FFI, will see the equivalent of the following C code:
//! ```c
//! void call_with_string(uint8_t* s);
//! ```
//!
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
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Default for AsciiPointer<'a> {
    fn default() -> Self {
        Self {
            ptr: null(),
            _phantom: Default::default(),
        }
    }
}

impl<'a> AsciiPointer<'a> {
    pub fn empty() -> Self {
        Self {
            ptr: EMPTY.as_ptr().cast(),
            _phantom: Default::default(),
        }
    }

    /// Create an AsciiPointer from a `&[u8]` slice reference.
    ///
    /// The parameter `ascii_with_nul` must contain nul (`0x0`), but it does not need to contain nul
    /// at the end.
    pub fn from_slice_with_nul(ascii_with_nul: &[u8]) -> Result<Self, Error> {
        // Check we actually contain one `0x0`.
        if !ascii_with_nul.contains(&0) {
            return Err(Error::Ascii);
        }

        // Can't do this, C# treats ASCII as extended and bytes > 127 might show up, which
        // is going to be a problem when returning a string we previously accepted.
        //
        // Any previous characters must not be extended ASCII.
        // if ascii_with_nul.iter().any(|x| *x > 127) {
        //     return Err(Error::Ascii);
        // }

        Ok(Self {
            ptr: ascii_with_nul.as_ptr().cast(),
            _phantom: Default::default(),
        })
    }

    /// Create a pointer from a CStr.
    pub fn from_cstr(cstr: &'a CStr) -> Self {
        Self {
            ptr: cstr.as_ptr(),
            _phantom: Default::default(),
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
    use crate::patterns::string::AsciiPointer;
    use std::ffi::CString;

    #[test]
    fn can_create() {
        let s = "hello world";
        let cstr = CString::new(s).unwrap();

        let ptr_some = AsciiPointer::from_cstr(&cstr);

        assert_eq!(s, ptr_some.as_str().unwrap());
    }

    #[test]
    fn from_slice_with_nul_works() {
        let s = b"hello\0world";
        let ptr_some = AsciiPointer::from_slice_with_nul(&s[..]).unwrap();

        assert_eq!("hello", ptr_some.as_str().unwrap());
    }

    #[test]
    fn from_slice_with_nul_fails_if_not_nul() {
        let s = b"hello world";
        let ptr_some = AsciiPointer::from_slice_with_nul(&s[..]);

        assert!(ptr_some.is_err());
    }
}
