use crate::types::common;
use interoptopus::ffi_function;
use interoptopus::patterns::option::FFIOption;
use interoptopus::patterns::slice::{FFISlice, FFISliceMut};

#[ffi_function]
pub fn namespaced_type(x: common::Vec) -> common::Vec {
    x
}

#[ffi_function]
pub fn namespaced_inner_option(x: FFIOption<common::Vec>) -> FFIOption<common::Vec> {
    x
}

#[ffi_function]
pub fn namespaced_inner_slice(x: FFISlice<common::Vec>) -> FFISlice<common::Vec> {
    x
}

#[ffi_function]
pub fn namespaced_inner_slice_mut(x: FFISliceMut<common::Vec>) -> FFISliceMut<common::Vec> {
    x
}
