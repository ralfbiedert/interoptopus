use interoptopus::{
    ffi, ffi_function, ffi_type,
    lang::{Wire, Wireable},
};

#[ffi_type(wired, debug)]
pub struct MyWiredType {
    name: String,
    values: Vec<u32>,
}

// input is a serialized representation, parse it to access MyWiredType.
// serialize resulting MyWiredType into a buffer and return it as WireOfMyWiredType on C# side
#[ffi_function]
fn perform_miracles(mut input: Wire<MyWiredType>) -> Wire<MyWiredType> {
    let w = input.unwire().expect("Something went wrong");
    w.wire()
}

#[ffi_function]
fn perform_half_miracles(mut input: Wire<MyWiredType>, other: ffi::String) -> ffi::String {
    let w = input.unwire().expect("Something went wrong");
    let result = format!("{} {}", w.name, other.as_str());
    result.into()
}

#[ffi_function]
fn perform_half_miracles_in_other_direction(input: ffi::String) -> Wire<'static, MyWiredType> {
    MyWiredType { name: input.as_str().to_owned(), values: vec![] }.wire()
}
