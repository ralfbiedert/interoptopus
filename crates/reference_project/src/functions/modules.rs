use interoptopus::ffi;
use crate::types::namespaces::common;
use interoptopus::pattern::option::Option;
use interoptopus::pattern::slice::{Slice, SliceMut};

// These functions use types that have a namespace attribute.

#[ffi]
pub fn namespaced_type(x: common::Vec) -> common::Vec {
    x
}

#[ffi]
pub fn namespaced_inner_option(x: Option<common::Vec>) -> Option<common::Vec> {
    x
}

#[ffi]
pub fn namespaced_inner_slice(x: Slice<common::Vec>) -> Slice<common::Vec> {
    x
}

#[ffi]
pub fn namespaced_inner_slice_mut(x: SliceMut<common::Vec>) -> SliceMut<common::Vec> {
    x
}
