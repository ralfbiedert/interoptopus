use interoptopus::{ffi_function, ffi_type};

#[ffi_type(wired)]
struct MyWiredType {
    name: String,
    values: Vec<u32>,
    attribute: (u32, String, HashMap<String, u8>),
}

#[ffi_function]
fn perform_miracles(input: Wire<MyWiredType>) -> Wire<MyWiredType> {}

#[ffi_function]
fn perform_half_miracles(input: Wire<MyWiredType>) -> ffi::String {}

#[ffi_function]
fn perform_half_miracles_in_other_direction(input: ffi::String) -> Wire<MyWiredType> {}
