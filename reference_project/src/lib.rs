//! A reference project for [Interoptopus](https://github.com/ralfbiedert/interoptopus).
//!
//! This project tries to use every Interoptopus feature at least once.
//! When submitting new features or making changes to existing ones the types and functions in
//! here will ensure existing backends still work as expected.
//!
//! Note, many items here are deliberately not documented as testing how and if documentation
//! is generated is part of the test.

use interoptopus::{constant, extra_type, function, pattern, Inventory, InventoryBuilder};

pub mod constants;
pub mod functions;
/// Reference implementations of patterns.
pub mod patterns {
    // pub mod api_entry;
    pub mod api_guard;
    pub mod ascii_pointer;
    pub mod callbacks;
    pub mod option;
    pub mod primitives;
    pub mod result;
    pub mod service;
    pub mod slice;
}
pub mod types;

pub fn ffi_inventory() -> Inventory {
    {
        InventoryBuilder::new()
            // Functions
            .register(function!(functions::primitive_void))
            .register(function!(functions::primitive_void2))
            .register(function!(functions::primitive_bool))
            .register(function!(functions::primitive_u8))
            .register(function!(functions::primitive_u16))
            .register(function!(functions::primitive_u32))
            .register(function!(functions::primitive_u64))
            .register(function!(functions::primitive_i8))
            .register(function!(functions::primitive_i16))
            .register(function!(functions::primitive_i32))
            .register(function!(functions::primitive_i64))
            .register(function!(functions::many_args_5))
            .register(function!(functions::many_args_10))
            .register(function!(functions::ptr))
            .register(function!(functions::ptr_mut))
            .register(function!(functions::ptr_ptr))
            .register(function!(functions::ref_simple))
            .register(function!(functions::ref_mut_simple))
            .register(function!(functions::ref_option))
            .register(function!(functions::ref_mut_option))
            .register(function!(functions::tupled))
            .register(function!(functions::complex_args_1))
            .register(function!(functions::complex_args_2))
            .register(function!(functions::callback))
            .register(function!(functions::generic_1a))
            .register(function!(functions::generic_1b))
            .register(function!(functions::generic_1c))
            .register(function!(functions::generic_2))
            .register(function!(functions::generic_3))
            .register(function!(functions::generic_4))
            .register(function!(functions::array_1))
            .register(function!(functions::documented))
            .register(function!(functions::ambiguous_1))
            .register(function!(functions::ambiguous_2))
            .register(function!(functions::ambiguous_3))
            .register(function!(functions::namespaced_type))
            .register(function!(functions::panics))
            .register(function!(functions::renamed))
            .register(function!(functions::sleep))
            .register(function!(functions::weird_1))
            .register(function!(functions::visibility))
            .register(function!(functions::repr_transparent))
            .register(function!(patterns::ascii_pointer::pattern_ascii_pointer_1))
            .register(function!(patterns::ascii_pointer::pattern_ascii_pointer_2))
            .register(function!(patterns::ascii_pointer::pattern_ascii_pointer_len))
            .register(function!(patterns::ascii_pointer::pattern_ascii_pointer_return_slice))
            .register(function!(patterns::slice::pattern_ffi_slice_1))
            .register(function!(patterns::slice::pattern_ffi_slice_2))
            .register(function!(patterns::slice::pattern_ffi_slice_3))
            .register(function!(patterns::slice::pattern_ffi_slice_4))
            .register(function!(patterns::slice::pattern_ffi_slice_5))
            .register(function!(patterns::slice::pattern_ffi_slice_6))
            .register(function!(patterns::slice::pattern_ffi_slice_delegate))
            .register(function!(patterns::slice::pattern_ffi_slice_delegate_huge))
            .register(function!(patterns::option::pattern_ffi_option_1))
            .register(function!(patterns::option::pattern_ffi_option_2))
            .register(function!(patterns::primitives::pattern_ffi_bool))
            .register(function!(patterns::primitives::pattern_ffi_cchar))
            .register(function!(patterns::primitives::pattern_ffi_cchar_const_pointer))
            .register(function!(patterns::primitives::pattern_ffi_cchar_mut_pointer))
            .register(function!(patterns::api_guard::pattern_api_guard))
            .register(function!(patterns::callbacks::pattern_callback_1))
            .register(function!(patterns::callbacks::pattern_callback_2))
            // Constants
            .register(constant!(constants::U8))
            .register(constant!(constants::F32_MIN_POSITIVE))
            .register(constant!(constants::COMPUTED_I32))
            // Extra Types
            .register(extra_type!(types::ExtraType<f32>))
            // Patterns
            .register(pattern!(patterns::service::SimpleService))
            .register(pattern!(patterns::service::SimpleServiceLifetime))
            .inventory()
    }
}
