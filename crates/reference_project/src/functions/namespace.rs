use crate::types::common;
use interoptopus::ffi_function;
use interoptopus::patterns::option::Option;
use interoptopus::patterns::slice::{Slice, SliceMut};

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
