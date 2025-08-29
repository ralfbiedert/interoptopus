//! A reference project for [Interoptopus](https://github.com/ralfbiedert/interoptopus).
//!
//! This project tries to use every Interoptopus feature at least once.
//! When submitting new features or making changes to existing ones the types and functions in
//! here will ensure existing backends still work as expected.
//!
//! Note, many items here are deliberately not documented as testing how and if documentation
//! is generated is part of the test.

use interoptopus::inventory::Inventory;
use interoptopus::{builtins_string, builtins_vec, constant, extra_type, ffi, function, pattern};

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
    pub mod slice;
    pub mod string;
    pub mod surrogate;
    pub mod vec;
}
pub mod services {
    pub mod asynk {
        pub mod basic;
        pub mod result;
        pub mod sleep;
        pub mod structs;
        pub mod todo_bad; // TODO - remove me later
        pub mod todo_threadlocal;
        pub mod vecstring;
    }
    pub mod basic;
    pub mod callback;
    pub mod dependent;
    pub mod ignored;
    pub mod multiple_ctors;
    pub mod on_panic;
    pub mod result;
    pub mod slice;
    pub mod string;
}

pub mod types;
mod wire;

pub fn ffi_inventory() -> Inventory {
    Inventory::builder()
        // Functions
        .register(builtins_string!())
        .register(builtins_vec!(u8))
        .register(builtins_vec!(ffi::String))
        .register(builtins_vec!(types::basic::Vec3f32))
        .register(builtins_vec!(types::enums::EnumPayload))
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
        .register(function!(functions::behavior::behavior_panics_via_result))
        .register(function!(functions::enums::enums_1))
        .register(function!(functions::enums::enums_2))
        .register(function!(functions::enums::enums_3))
        .register(function!(functions::enums::enums_4))
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
        .register(function!(functions::modules::namespaced_inner_option))
        .register(function!(functions::modules::namespaced_inner_slice))
        .register(function!(functions::modules::namespaced_inner_slice_mut))
        .register(function!(functions::modules::namespaced_type))
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
        .register(function!(functions::primitive::primitive_usize))
        .register(function!(functions::primitive::primitive_isize))
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
        .register(function!(functions::refs::ref5))
        .register(function!(functions::refs::ref6))
        .register(function!(functions::refs::ref7))
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
        .register(function!(patterns::string::pattern_string_9))
        .register(function!(patterns::string::pattern_string_10))
        .register(function!(patterns::string::pattern_string_11))
        .register(function!(patterns::slice::pattern_ffi_slice_1))
        .register(function!(patterns::slice::pattern_ffi_slice_1b))
        .register(function!(patterns::slice::pattern_ffi_slice_2))
        .register(function!(patterns::slice::pattern_ffi_slice_3))
        .register(function!(patterns::slice::pattern_ffi_slice_4))
        .register(function!(patterns::slice::pattern_ffi_slice_5))
        .register(function!(patterns::slice::pattern_ffi_slice_6))
        // .register(function!(patterns::slice::pattern_ffi_slice_7))
        .register(function!(patterns::slice::pattern_ffi_slice_8))
        .register(function!(patterns::slice::pattern_ffi_slice_9))
        .register(function!(patterns::slice::pattern_ffi_slice_delegate))
        .register(function!(patterns::slice::pattern_ffi_slice_delegate_huge))
        .register(function!(patterns::option::pattern_ffi_option_1))
        .register(function!(patterns::option::pattern_ffi_option_2))
        .register(function!(patterns::option::pattern_ffi_option_3))
        .register(function!(patterns::primitive::pattern_ffi_bool))
        .register(function!(patterns::primitive::pattern_ffi_cchar))
        .register(function!(patterns::primitive::pattern_ffi_cchar_const_pointer))
        .register(function!(patterns::primitive::pattern_ffi_cchar_mut_pointer))
        .register(function!(patterns::result::pattern_result_1))
        .register(function!(patterns::result::pattern_result_2))
        .register(function!(patterns::result::pattern_result_3))
        .register(function!(patterns::result::pattern_result_4))
        .register(function!(patterns::api_guard::pattern_api_guard))
        .register(function!(patterns::callback::pattern_callback_1))
        .register(function!(patterns::callback::pattern_callback_2))
        // .register(function!(patterns::callback::pattern_callback_3))
        .register(function!(patterns::callback::pattern_callback_4))
        .register(function!(patterns::callback::pattern_callback_5))
        .register(function!(patterns::callback::pattern_callback_6))
        .register(function!(patterns::callback::pattern_callback_7))
        .register(function!(patterns::callback::pattern_callback_8))
        .register(function!(patterns::surrogate::pattern_surrogates_1))
        .register(function!(patterns::vec::pattern_vec_1))
        .register(function!(patterns::vec::pattern_vec_2))
        .register(function!(patterns::vec::pattern_vec_3))
        .register(function!(patterns::vec::pattern_vec_4))
        .register(function!(patterns::vec::pattern_vec_5))
        .register(function!(patterns::vec::pattern_vec_6))
        .register(function!(patterns::vec::pattern_vec_7))
        .register(function!(patterns::vec::pattern_vec_8))
        // TODO
        // .register(function!(wire::basic::perform_miracles))
        // .register(function!(wire::basic::perform_half_miracles))
        // .register(function!(wire::basic::perform_half_miracles_in_other_direction))
        // Constants
        .register(constant!(constants::U8))
        .register(constant!(constants::F32_MIN_POSITIVE))
        .register(constant!(constants::COMPUTED_I32))
        // Extra Types
        .register(extra_type!(types::generic::ExtraType<f32>))
        .register(extra_type!(types::num::EnumNum))
        .register(extra_type!(types::num::IVec3))
        .register(extra_type!(types::num::TransparentNum))
        // Services
        .register(pattern!(services::asynk::basic::ServiceAsyncBasic))
        .register(pattern!(services::asynk::sleep::ServiceAsyncSleep))
        .register(pattern!(services::asynk::vecstring::ServiceAsyncVecString))
        .register(pattern!(services::asynk::result::ServiceAsyncResult))
        .register(pattern!(services::asynk::structs::ServiceAsyncStructs))
        .register(pattern!(services::basic::ServiceBasic))
        .register(pattern!(services::dependent::ServiceMain))
        .register(pattern!(services::dependent::ServiceDependent))
        .register(pattern!(services::result::ServiceResult))
        .register(pattern!(services::on_panic::ServiceOnPanic))
        .register(pattern!(services::callback::ServiceCallbacks))
        .register(pattern!(services::ignored::ServiceIgnoringMethods))
        .register(pattern!(services::multiple_ctors::ServiceMultipleCtors))
        .register(pattern!(services::slice::ServiceVariousSlices))
        .register(pattern!(services::string::ServiceStrings))
        .validate()
        .build()
}
