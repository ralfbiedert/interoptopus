use crate::types::namespaces::common;
use interoptopus::ffi_function;
use interoptopus::pattern::option::Option;
use interoptopus::pattern::slice::{Slice, SliceMut};

#[ffi_function]
pub fn namespaced_type(x: common::Vec) -> common::Vec {
    x
}

#[ffi_function]
pub fn namespaced_inner_option(x: Option<common::Vec>) -> Option<common::Vec> {
    x
}

#[ffi_function]
pub fn namespaced_inner_slice(x: Slice<common::Vec>) -> Slice<common::Vec> {
    x
}

#[ffi_function]
pub fn namespaced_inner_slice_mut(x: SliceMut<common::Vec>) -> SliceMut<common::Vec> {
    x
}
