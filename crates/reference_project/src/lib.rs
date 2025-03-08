//! A reference project for [Interoptopus](https://github.com/ralfbiedert/interoptopus).
//!
//! This project tries to use every Interoptopus feature at least once.
//! When submitting new features or making changes to existing ones the types and functions in
//! here will ensure existing backends still work as expected.
//!
//! Note, many items here are deliberately not documented as testing how and if documentation
//! is generated is part of the test.

use interoptopus::{builtins, constant, extra_type, function, pattern, Inventory, InventoryBuilder};

pub mod constants;
pub mod functions;
/// Reference implementations of patterns.
pub mod patterns {
    // pub mod api_entry;
    pub mod api_guard;
    pub mod callback;
    pub mod option;
    pub mod primitive;
    pub mod result;
    pub mod string;
    pub mod services {
        pub mod _tmp;
        pub mod asynk;
        pub mod basic;
        pub mod callback;
        pub mod converter;
        pub mod ignored;
        pub mod multiple_ctors;
        pub mod on_panic;
        pub mod result;
        pub mod slice;
        pub mod string;
    }
    pub mod slice;
    pub mod surrogate;
}
pub mod types;

pub fn ffi_inventory() -> Inventory {
    {
        InventoryBuilder::new()
            // Functions
            .register(builtins!())
            .register(function!(functions::alignment::alignment_1))
            .register(function!(functions::array::array_1))
            .register(function!(functions::array::array_2))
            .register(function!(functions::array::array_3))
            .register(function!(functions::array::char_array_1))
            .register(function!(functions::array::char_array_2))
            .register(function!(functions::array::char_array_3))
            .register(function!(functions::array::nested_array_1))
            .register(function!(functions::array::nested_array_2))
            .register(function!(functions::array::nested_array_3))
            .register(function!(functions::behavior::behavior_sleep))
            .register(function!(functions::behavior::behavior_panics))
            .register(function!(functions::fnptrs::fnptr_1))
            .register(function!(functions::fnptrs::fnptr_2))
            .register(function!(functions::generic::generic_1a))
            .register(function!(functions::generic::generic_1b))
            .register(function!(functions::generic::generic_1c))
            .register(function!(functions::generic::generic_2))
            .register(function!(functions::generic::generic_3))
            .register(function!(functions::generic::generic_4))
            .register(function!(functions::generic::generic_5))
            .register(function!(functions::meta::meta_ambiguous_1))
            .register(function!(functions::meta::meta_ambiguous_2))
            .register(function!(functions::meta::meta_ambiguous_3))
            .register(function!(functions::meta::meta_documented))
            .register(function!(functions::meta::meta_visibility1))
            .register(function!(functions::meta::meta_renamed))
            .register(function!(functions::namespace::namespaced_inner_option))
            .register(function!(functions::namespace::namespaced_inner_slice))
            .register(function!(functions::namespace::namespaced_inner_slice_mut))
            .register(function!(functions::namespace::namespaced_type))
            .register(function!(functions::primitive::primitive_args_5))
            .register(function!(functions::primitive::primitive_args_10))
            .register(function!(functions::primitive::primitive_bool))
            .register(function!(functions::primitive::primitive_f32))
            .register(function!(functions::primitive::primitive_f64))
            .register(function!(functions::primitive::primitive_i16))
            .register(function!(functions::primitive::primitive_i32))
            .register(function!(functions::primitive::primitive_i64))
            .register(function!(functions::primitive::primitive_i8))
            .register(function!(functions::primitive::primitive_u16))
            .register(function!(functions::primitive::primitive_u32))
            .register(function!(functions::primitive::primitive_u64))
            .register(function!(functions::primitive::primitive_u8))
            .register(function!(functions::primitive::primitive_void))
            .register(function!(functions::primitive::primitive_void2))
            .register(function!(functions::ptrs::ptr1))
            .register(function!(functions::ptrs::ptr2))
            .register(function!(functions::ptrs::ptr3))
            .register(function!(functions::refs::ref1))
            .register(function!(functions::refs::ref2))
            .register(function!(functions::refs::ref3))
            .register(function!(functions::refs::ref4))
            .register(function!(functions::structs::struct1))
            .register(function!(functions::structs::struct2))
            .register(function!(functions::structs::struct3))
            .register(function!(patterns::string::pattern_ascii_pointer_1))
            .register(function!(patterns::string::pattern_ascii_pointer_2))
            .register(function!(patterns::string::pattern_ascii_pointer_3))
            .register(function!(patterns::string::pattern_ascii_pointer_4))
            .register(function!(patterns::string::pattern_ascii_pointer_5))
            .register(function!(patterns::string::pattern_ascii_pointer_return_slice))
            .register(function!(patterns::string::pattern_string_1))
            .register(function!(patterns::string::pattern_string_2))
            .register(function!(patterns::string::pattern_string_3))
            .register(function!(patterns::string::pattern_string_4))
            .register(function!(patterns::string::pattern_string_5))
            .register(function!(patterns::string::pattern_string_6a))
            .register(function!(patterns::string::pattern_string_6b))
            .register(function!(patterns::string::pattern_string_7))
            .register(function!(patterns::string::pattern_string_8))
            .register(function!(patterns::slice::pattern_ffi_slice_1))
            .register(function!(patterns::slice::pattern_ffi_slice_1b))
            .register(function!(patterns::slice::pattern_ffi_slice_2))
            .register(function!(patterns::slice::pattern_ffi_slice_3))
            .register(function!(patterns::slice::pattern_ffi_slice_4))
            .register(function!(patterns::slice::pattern_ffi_slice_5))
            .register(function!(patterns::slice::pattern_ffi_slice_6))
            // .register(function!(patterns::slice::pattern_ffi_slice_7))
            .register(function!(patterns::slice::pattern_ffi_slice_8))
            .register(function!(patterns::slice::pattern_ffi_slice_delegate))
            .register(function!(patterns::slice::pattern_ffi_slice_delegate_huge))
            .register(function!(patterns::option::pattern_ffi_option_1))
            .register(function!(patterns::option::pattern_ffi_option_2))
            .register(function!(patterns::primitive::pattern_ffi_bool))
            .register(function!(patterns::primitive::pattern_ffi_cchar))
            .register(function!(patterns::primitive::pattern_ffi_cchar_const_pointer))
            .register(function!(patterns::primitive::pattern_ffi_cchar_mut_pointer))
            .register(function!(patterns::result::pattern_result_1))
            .register(function!(patterns::result::pattern_result_2))
            .register(function!(patterns::result::pattern_result_3))
            .register(function!(patterns::api_guard::pattern_api_guard))
            .register(function!(patterns::callback::pattern_callback_1))
            .register(function!(patterns::callback::pattern_callback_2))
            // .register(function!(patterns::callback::pattern_callback_3))
            .register(function!(patterns::callback::pattern_callback_4))
            .register(function!(patterns::callback::pattern_callback_5))
            .register(function!(patterns::callback::pattern_callback_6))
            .register(function!(patterns::callback::pattern_callback_7))
            .register(function!(patterns::surrogate::pattern_surrogates_1))
            // Constants
            .register(constant!(constants::U8))
            .register(constant!(constants::F32_MIN_POSITIVE))
            .register(constant!(constants::COMPUTED_I32))
            // Extra Types
            .register(extra_type!(types::ExtraType<f32>))
            // Patterns
            .register(pattern!(patterns::services::asynk::ServiceAsync))
            .register(pattern!(patterns::services::basic::ServiceBasic))
            .register(pattern!(patterns::services::result::ServiceResult))
            .register(pattern!(patterns::services::on_panic::ServiceOnPanic))
            .register(pattern!(patterns::services::callback::ServiceCallbacks))
            .register(pattern!(patterns::services::ignored::ServiceIgnoringMethods))
            .register(pattern!(patterns::services::multiple_ctors::ServiceMultipleCtors))
            .register(pattern!(patterns::services::slice::ServiceVariousSlices))
            .register(pattern!(patterns::services::string::ServiceStrings))
            .validate()
            .build()
    }
}
