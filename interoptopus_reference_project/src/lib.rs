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
        functions::callback
    ]
);
