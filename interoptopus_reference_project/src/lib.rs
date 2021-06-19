use interoptopus::pattern_class;

pub mod constants;
pub mod functions;
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
        functions::complex_1,
        functions::complex_2,
        functions::callback,
        functions::generic,
        functions::documented,
        functions::pattern_ascii_pointer,
        functions::pattern_class_create,
        functions::pattern_class_method,
        functions::pattern_class_destroy,
        functions::pattern_class_method_success_enum_ok,
        functions::pattern_class_method_success_enum_fail,
        functions::pattern_ffi_slice,
        functions::pattern_ffi_slice_delegate,
        functions::ambiguous_1,
        functions::ambiguous_2,
        functions::ambiguous_3
    ],
    [my_class_pattern_context]
);

pattern_class!(
    my_class_pattern_context,
    functions::pattern_class_create,
    functions::pattern_class_destroy,
    [
        functions::pattern_class_method,
        functions::pattern_class_method_success_enum_ok,
        functions::pattern_class_method_success_enum_fail
    ]
);
