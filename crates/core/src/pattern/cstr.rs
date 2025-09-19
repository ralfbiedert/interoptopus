//! Pointer like `*const char` that becomes `string` in supported languages.
//!
//! # Example
//!
//! In your library you can accept (ASCII- / C-) strings like this:
//!
//! ```
//! use interoptopus::ffi_function;
//! use interoptopus::ffi;
//!
//! #[ffi_function]
//! pub extern "C" fn call_with_string(s: ffi::CStrPtr) {
//!     //
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
use crate::inventory::{Inventory, TypeId};
use crate::lang::meta::{Docs, Emission, Visibility};
use crate::lang::types::{Type, TypeInfo, TypeKind, TypePattern};
use crate::Error;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::option::Option::None;
use std::os::raw::c_char;
use std::ptr::null;

static EMPTY: &[u8] = b"\0";

/// Represents a `*const char` on FFI level pointing to an `0x0` terminated ASCII string.
///
/// # Antipattern
///
/// It's discouraged to use [`FFIOption`](crate::pattern::option::Option) with [`CStrPtr`]
/// and some backend might not generate proper bindings (like C#).
///
/// Instead use [`CStrPtr`] alone since it already has a pointer that's nullable.
/// In this case, [`CStrPtr::as_c_str()`] will return [`None`] and [`CStrPtr::as_str`]
/// will return an [`Error::Null`].
#[repr(transparent)]
#[derive(Debug)]
pub struct CStrPtr<'a> {
    ptr: *const c_char,
    _phantom: PhantomData<&'a ()>,
}

// Safety: `CStrPointer` is a transparent wrapper around a pointer. From Rust
//         we only allow safe construction, from interop it's up to the FFI caller.
unsafe impl Send for CStrPtr<'_> {}
unsafe impl Sync for CStrPtr<'_> {}

impl Default for CStrPtr<'_> {
    fn default() -> Self {
        Self { ptr: null(), _phantom: PhantomData::default() }
    }
}

impl<'a> CStrPtr<'a> {
    #[must_use]
    pub fn empty() -> Self {
        Self { ptr: EMPTY.as_ptr().cast(), _phantom: PhantomData::default() }
    }

    /// Create a `CStrPointer` from a `&[u8]` slice reference.
    ///
    /// The parameter `cstr_with_nul` must contain nul (`0x0`), but it does not need to contain nul
    /// at the end.
    ///
    /// # Errors
    /// Can fail if the string contains a nul.
    pub fn from_slice_with_nul(cstr_with_nul: &'a [u8]) -> Result<Self, Error> {
        // Check we actually contain one `0x0`.
        if !cstr_with_nul.contains(&0) {
            return Err(Error::NulTerminated);
        }

        // Can't do this, C# treats ASCII as extended and bytes > 127 might show up, which
        // is going to be a problem when returning a string we previously accepted.
        //
        // Any previous characters must not be extended ASCII.
        // if ascii_with_nul.iter().any(|x| *x > 127) {
        //     return Err(Error::Ascii);
        // }

        Ok(Self { ptr: cstr_with_nul.as_ptr().cast(), _phantom: PhantomData::default() })
    }

    /// Create a pointer from a `CStr`.
    #[must_use]
    pub fn from_cstr(cstr: &'a CStr) -> Self {
        Self { ptr: cstr.as_ptr(), _phantom: PhantomData::default() }
    }

    /// Create a [`CStr`] for the pointer.
    #[must_use]
    pub fn as_c_str(&self) -> Option<&'a CStr> {
        if self.ptr.is_null() {
            None
        } else {
            // TODO: Write something about safety
            unsafe { Some(CStr::from_ptr(self.ptr)) }
        }
    }

    /// Attempts to return a Rust `str`.
    ///
    /// # Errors
    /// Can fail if the string was null.
    pub fn as_str(&self) -> Result<&'a str, Error> {
        Ok(self.as_c_str().ok_or(Error::Null)?.to_str()?)
    }
}

impl TypeInfo for CStrPtr<'_> {
    const WIRE_SAFE: bool = true;
    const RAW_SAFE: bool = true;

    fn id() -> TypeId {
        TypeId::new(0xDE450364E9ADDBA5DC9A6C5BBEC7759F)
    }

    fn kind() -> TypeKind {
        TypeKind::TypePattern(TypePattern::CStrPointer)
    }

    fn ty() -> Type {
        Type {
            emission: Emission::Common,
            docs: Docs::empty(),
            visibility: Visibility::Public,
            name: "CStrPtr".to_string(),
            kind: Self::kind(),
        }
    }

    fn register(inventory: &mut Inventory) {
        inventory.register_type(Self::id(), Self::ty());
    }
}

impl crate::lang::Register for CStrPtr<'_> {
    fn register(inventory: &mut Inventory) {
        <Self as TypeInfo>::register(inventory);
    }
}

#[cfg(test)]
mod test {
    use crate::pattern::cstr::CStrPtr;
    use std::ffi::CString;

    #[test]
    fn can_create() {
        let s = "hello world";
        let cstr = CString::new(s).unwrap();

        let ptr_some = CStrPtr::from_cstr(&cstr);

        assert_eq!(s, ptr_some.as_str().unwrap());
    }

    #[test]
    fn from_slice_with_nul_works() {
        let s = b"hello\0world";
        let ptr_some = CStrPtr::from_slice_with_nul(&s[..]).unwrap();

        assert_eq!("hello", ptr_some.as_str().unwrap());
    }

    #[test]
    fn from_slice_with_nul_fails_if_not_nul() {
        let s = b"hello world";
        let ptr_some = CStrPtr::from_slice_with_nul(&s[..]);

        assert!(ptr_some.is_err());
    }
}
