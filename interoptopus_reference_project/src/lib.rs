//! A reference project for [Interoptopus](https://github.com/ralfbiedert/interoptopus).
//!
//! This project tries to use every Interoptopus feature at least once.
//! When submitting new features or making changes to existing ones the types and functions in
//! here will ensure existing backends still work as expected.
//!
//! Note, many items here are deliberately not documented as testing how and if documentation
//! is generated is part of the test.

pub mod constants;
pub mod functions;
pub mod patterns {
    pub mod class_generated;
    pub mod class_manual;
}
pub mod types;

interoptopus::inventory_function!(
    ffi_inventory,
    [constants::C1, constants::C2, constants::C3],
    [
        functions::primitive_void,
        functions::primitive_void2,
        functions::primitive_bool,
        functions::primitive_u8,
        functions::primitive_u16,
        functions::primitive_u32,
        functions::primitive_u64,
        functions::primitive_i8,
        functions::primitive_i16,
        functions::primitive_i32,
        functions::primitive_i64,
        functions::ptr,
        functions::ptr_mut,
        functions::ptr_ptr,
        functions::ptr_simple,
        functions::ptr_simple_mut,
        functions::ptr_option,
        functions::ptr_option_mut,
        functions::tupled,
        functions::complex_1,
        functions::complex_2,
        functions::callback,
        functions::generic_1,
        functions::generic_2,
        functions::documented,
        functions::pattern_ascii_pointer,
        functions::pattern_ffi_slice,
        functions::pattern_ffi_slice_delegate,
        functions::ambiguous_1,
        functions::ambiguous_2,
        functions::ambiguous_3,
        functions::namespaced_type
    ],
    [patterns::class_manual::my_class_pattern_context, patterns::class_generated::simple_class_pattern]
);
