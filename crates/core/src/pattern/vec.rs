//! Like a regular [`Vec`](std::vec::Vec), but FFI safe.<sup>🚧</sup>
use crate::inventory::{Inventory, TypeId};
use crate::lang::meta::{Emission, Visibility};
use crate::lang::types::{Type, TypeInfo, TypeKind, TypePattern};
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

impl<T: TypeInfo> TypeInfo for Vec<T> {
    const WIRE_SAFE: bool = false;
    const RAW_SAFE: bool = true;
    const ASYNC_SAFE: bool = true;
    const SERVICE_SAFE: bool = false;
    const SERVICE_CTOR_SAFE: bool = false;

    fn id() -> TypeId {
        TypeId::new(0x8046F09DA6F9059E4A0D8327DDE7FAC8).derive_id(T::id())
    }

    fn kind() -> TypeKind {
        TypeKind::TypePattern(TypePattern::Vec(T::id()))
    }

    fn ty() -> Type {
        let t = T::ty();
        Type {
            name: format!("Vec<{}>", t.name),
            visibility: Visibility::Public,
            docs: Default::default(),
            emission: if matches!(t.emission, Emission::Module(_)) { t.emission } else { Emission::Common },
            kind: Self::kind(),
        }
    }

    fn register(inventory: &mut Inventory) {
        T::register(inventory);
        inventory.register_type(Self::id(), Self::ty());
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

/// Emits helper functions used by [`Vec`](crate::pattern::vec::Vec).
#[macro_export]
macro_rules! builtins_vec {
    ($t:ty) => {{
        #[$crate::ffi_function(export = unique)]
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

        #[$crate::ffi_function(export = unique)]
        pub fn interoptopus_vec_destroy(_: $crate::ffi::Vec<$t>) -> i64 {
            0
        }

        |x: &mut $crate::inventory::Inventory| {
            <interoptopus_vec_create as $crate::lang::function::FunctionInfo>::register(x);
            <interoptopus_vec_destroy as $crate::lang::function::FunctionInfo>::register(x);
        }
    }};
}
