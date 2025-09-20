//! Like a regular [`String`](std::string::String), but FFI safe.

use crate::inventory::{Inventory, TypeId};
use crate::lang::meta::{Emission, Visibility};
use crate::lang::types::{Type, TypeInfo, TypeKind, TypePattern};
use std::mem::forget;

/// FFI analog of [`std::string::String`].
#[derive(Debug)]
#[repr(C)]
pub struct String {
    ptr: *mut u8,
    len: u64,
    capacity: u64,
}

unsafe impl Send for String {}
unsafe impl Sync for String {}

impl String {
    #[must_use]
    pub fn from_string(mut s: std::string::String) -> Self {
        let ptr = s.as_mut_ptr();
        let capacity = s.capacity() as u64;
        let len = s.len() as u64;
        forget(s);
        Self { ptr, len, capacity }
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn as_str(&self) -> &str {
        if self.ptr.is_null() {
            return "";
        }

        unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.ptr, self.len as usize)) }
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn into_string(self) -> std::string::String {
        let rval = unsafe { std::string::String::from_raw_parts(self.ptr, self.len as usize, self.capacity as usize) };
        forget(self);
        rval
    }
}

impl From<std::string::String> for String {
    fn from(value: std::string::String) -> Self {
        Self::from_string(value)
    }
}

impl From<String> for std::string::String {
    fn from(value: String) -> Self {
        value.into_string()
    }
}

impl Clone for String {
    fn clone(&self) -> Self {
        Self::from_string(self.as_str().to_string())
    }
}

impl Drop for String {
    #[allow(clippy::cast_possible_truncation)]
    fn drop(&mut self) {
        if self.ptr.is_null() {
            return;
        }
        unsafe {
            let _ = std::string::String::from_raw_parts(self.ptr, self.len as usize, self.capacity as usize);
        }
    }
}

impl TypeInfo for String {
    const WIRE_SAFE: bool = false;
    const RAW_SAFE: bool = true;
    const ASYNC_SAFE: bool = true;

    fn id() -> TypeId {
        TypeId::new(0x0D49712411310AE6E26AD32245BF70B2)
    }

    fn kind() -> TypeKind {
        TypeKind::TypePattern(TypePattern::Utf8String)
    }

    fn ty() -> Type {
        Type { emission: Emission::Common, docs: crate::lang::meta::Docs::empty(), visibility: Visibility::Public, name: "String".to_string(), kind: Self::kind() }
    }

    fn register(inventory: &mut Inventory) {
        inventory.register_type(Self::id(), Self::ty());
    }
}

/// Emits helper functions used by [`String`](crate::pattern::string::String).
#[macro_export]
macro_rules! builtins_string {
    () => {{
        use ::interoptopus::lang::FunctionInfo;

        #[$crate::ffi_function]
        pub fn interoptopus_string_create(utf8: *const ::std::ffi::c_void, len: u64, rval: &mut ::std::mem::MaybeUninit<$crate::pattern::string::String>) -> i64 {
            let slice = if utf8.is_null() {
                &[]
            } else {
                unsafe { ::std::slice::from_raw_parts::<u8>(utf8.cast(), len as usize) }
            };
            let vec = slice.to_vec();
            let string = unsafe { ::std::string::String::from_utf8_unchecked(vec) };
            rval.write($crate::pattern::string::String::from_string(string));
            0
        }

        #[$crate::ffi_function]
        pub fn interoptopus_string_destroy(utf8: $crate::pattern::string::String) -> i64 {
            0
        }

        #[$crate::ffi_function]
        pub fn interoptopus_string_clone(utf8: &$crate::pattern::string::String, rval: &mut ::std::mem::MaybeUninit<$crate::pattern::string::String>) -> i64 {
            rval.write(utf8.clone());
            0
        }

        let items = vec![
            interoptopus_string_create::function_info(),
            interoptopus_string_destroy::function_info(),
            interoptopus_string_clone::function_info(),
        ];
        let builtins = $crate::pattern::builtins::Builtins::new(items);
        let pattern = $crate::pattern::LibraryPattern::Builtins(builtins);
        $crate::inventory::Symbol::Pattern(pattern)
    }};
}
