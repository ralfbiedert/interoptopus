//! Like a regular [`Vec`](std::vec::Vec), but FFI safe.<sup>🚧</sup>
use crate::lang::util::capitalize_first_letter;
use crate::lang::{Composite, Docs, Field, Layout, Meta, Primitive, Representation, Type, TypeInfo};
use crate::pattern::TypePattern;
use std::mem::forget;

#[derive(Debug)]
#[repr(C)]
pub struct Vec<T> {
    ptr: *mut T,
    len: u64,
    capacity: u64,
}

unsafe impl<T> Send for Vec<T> where T: Send {}
unsafe impl<T> Sync for Vec<T> where T: Sync {}

impl<T> Vec<T> {
    #[must_use]
    pub fn from_vec(mut s: std::vec::Vec<T>) -> Self {
        let ptr = s.as_mut_ptr();
        let capacity = s.capacity() as u64;
        let len = s.len() as u64;
        forget(s);
        Self { ptr, len, capacity }
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn len(&self) -> usize {
        self.len as usize
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn into_vec(self) -> std::vec::Vec<T> {
        let rval = unsafe { std::vec::Vec::from_raw_parts(self.ptr, self.len as usize, self.capacity as usize) };
        forget(self);
        rval
    }
}

impl<T: Clone + TypeInfo> Clone for Vec<T> {
    #[allow(clippy::cast_possible_truncation)]
    fn clone(&self) -> Self {
        let this = unsafe { std::vec::Vec::from_raw_parts(self.ptr, self.len as usize, self.capacity as usize) };
        let rval = this.clone();
        forget(this);
        rval.into()
    }
}

impl<T: TypeInfo> From<std::vec::Vec<T>> for Vec<T> {
    fn from(value: std::vec::Vec<T>) -> Self {
        Self::from_vec(value)
    }
}

impl<T: TypeInfo> From<Vec<T>> for std::vec::Vec<T> {
    fn from(value: Vec<T>) -> Self {
        value.into_vec()
    }
}

impl<T> Drop for Vec<T> {
    #[allow(clippy::cast_possible_truncation)]
    fn drop(&mut self) {
        if self.ptr.is_null() {
            return;
        }
        unsafe {
            let _ = std::vec::Vec::from_raw_parts(self.ptr, self.len as usize, self.capacity as usize);
        }
    }
}

unsafe impl<T> TypeInfo for Vec<T>
where
    T: TypeInfo,
{
    #[rustfmt::skip]
    fn type_info() -> Type {
        let fields = vec![
            Field::new("ptr".to_string(), Type::ReadWritePointer(Box::new(T::type_info()))),
            Field::new("len".to_string(), Type::Primitive(Primitive::U64)),
            Field::new("capacity".to_string(), Type::Primitive(Primitive::U64)),
        ];

        let doc = Docs::from_lines(vec![
            " Vec marshalling helper.".to_string(),
            " A highly dangerous 'use once type' that has ownership semantics!".to_string(),
            " Once passed over an FFI boundary 'the other side' is meant to own".to_string(),
            " (and free) it. Rust handles that fine, but if in C# you put this".to_string(),
            " in a struct and then call Rust multiple times with that struct ".to_string(),
            " you'll free the same pointer multiple times, and get UB!".to_string(),
        ]);
        let repr = Representation::new(Layout::C, None);
        let meta = Meta::with_docs(doc);
        let name = capitalize_first_letter(T::type_info().name_within_lib().as_str());
        let composite = Composite::with_meta_repr(format!("Vec{name}"), fields, meta, repr);
        let vec_type = VecType::new(composite, Box::new(T::type_info()));
        Type::Pattern(TypePattern::Vec(vec_type))
    }
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct VecType {
    composite_type: Composite,
    target_type: Box<Type>,
}

impl VecType {
    #[must_use]
    pub const fn new(composite_type: Composite, target_type: Box<Type>) -> Self {
        Self { composite_type, target_type }
    }

    #[must_use]
    pub fn rust_name(&self) -> &str {
        self.composite_type.rust_name()
    }

    #[must_use]
    pub const fn composite_type(&self) -> &Composite {
        &self.composite_type
    }

    #[must_use]
    pub const fn t(&self) -> &Type {
        &self.target_type
    }

    #[must_use]
    pub const fn meta(&self) -> &Meta {
        self.composite_type.meta()
    }

    #[must_use]
    pub fn to_type(&self) -> Type {
        Type::Pattern(TypePattern::Vec(self.clone()))
    }
}

/// Emits helper functions used by [`Vec`](crate::pattern::vec::Vec).
#[macro_export]
macro_rules! builtins_vec {
    ($t:ty) => {{
        use ::interoptopus::lang::FunctionInfo;

        #[$crate::ffi_function(export_unique)]
        pub fn interoptopus_vec_create(data: *const ::std::ffi::c_void, len: u64, rval: &mut ::std::mem::MaybeUninit<$crate::pattern::vec::Vec<$t>>) -> i64 {
            let slice = if data.is_null() {
                &[]
            } else {
                unsafe { ::std::slice::from_raw_parts::<$t>(data.cast(), len as usize) }
            };
            let vec = slice.to_vec();
            rval.write($crate::pattern::vec::Vec::from_vec(vec));
            0
        }

        #[$crate::ffi_function(export_unique)]
        pub fn interoptopus_vec_destroy(_: $crate::ffi::Vec<$t>) -> i64 {
            0
        }

        let functions = vec![interoptopus_vec_create::function_info(), interoptopus_vec_destroy::function_info()];
        let builtins = $crate::pattern::builtins::Builtins::new(functions);
        let pattern = $crate::pattern::LibraryPattern::Builtins(builtins);
        $crate::inventory::Symbol::Pattern(pattern)
    }};
}
