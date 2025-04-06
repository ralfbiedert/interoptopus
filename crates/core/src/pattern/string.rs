//! Like a regular [`String`](std::string::String), but FFI safe.

use crate::lang::{Composite, Docs, Field, Meta, Primitive, Representation, Type};
use crate::lang::{Layout, TypeInfo};
use crate::pattern::TypePattern;
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

unsafe impl TypeInfo for String {
    #[rustfmt::skip]
    fn type_info() -> Type {
        let fields = vec![
            Field::new("ptr".to_string(), Type::ReadWritePointer(Box::new(Type::Primitive(Primitive::U8)))),
            Field::new("len".to_string(), Type::Primitive(Primitive::U64)),
            Field::new("capacity".to_string(), Type::Primitive(Primitive::U64)),
        ];

        let doc = Docs::from_lines(vec![
            " UTF-8 string marshalling helper.".to_string(),
            " A highly dangerous 'use once type' that has ownership semantics!".to_string(),
            " Once passed over an FFI boundary 'the other side' is meant to own".to_string(),
            " (and free) it. Rust handles that fine, but if in C# you put this".to_string(),
            " in a struct and then call Rust multiple times with that struct ".to_string(),
            " you'll free the same pointer multiple times, and get UB!".to_string(),
        ]);
        let repr = Representation::new(Layout::C, None);
        let meta = Meta::with_docs(doc);
        let composite = Composite::with_meta_repr("Utf8String".to_string(), fields, meta, repr);
        Type::Pattern(TypePattern::Utf8String(composite))
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
