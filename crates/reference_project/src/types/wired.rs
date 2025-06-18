// use interoptopus::{ffi, ffi_function, ffi_type, pattern::wire::Wire};
use interoptopus::ffi_type;

#[ffi_type(wired, debug)]
struct MyWiredType {
    name: String,
    values: Vec<u32>,
    // attribute: (u32, String, HashMap<String, u8>), Interopopus does not support tuples yet
}
/*
// input is a serialized representation, parse it to access MyWiredType.
// serialize resulting MyWiredType into a buffer and return it as WireOfMyWiredType on C# side
#[ffi_function]
fn perform_miracles(input: Wire<MyWiredType>) -> Wire<MyWiredType> {}

#[ffi_function]
fn perform_half_miracles(input: Wire<MyWiredType>, other: ffi::String) -> ffi::String {}

#[ffi_function]
fn perform_half_miracles_in_other_direction(input: ffi::String) -> Wire<MyWiredType> {}
*/
