use interoptopus::ffi;
use interoptopus::lang::types::WireIO;
use interoptopus::wire::Wire;

#[ffi]
pub struct MyWiredType {
    name: String,
    values: Vec<u32>,
}

// input is a serialized representation, parse it to access MyWiredType.
// serialize resulting MyWiredType into a buffer and return it as WireOfMyWiredType on C# side
#[ffi]
pub fn perform_miracles(mut input: Wire<MyWiredType>) -> Wire<MyWiredType> {
    let w = input.unwire().expect("Something went wrong");
    let mut out = Wire::with_size(w.live_size());
    out.serialize(&w).unwrap();
    out
}

#[ffi]
pub fn perform_half_miracles(mut input: Wire<MyWiredType>, other: ffi::String) -> ffi::String {
    let w = input.unwire().expect("Something went wrong");
    let result = format!("{} {}", w.name, other.as_str());
    result.into()
}

// #[ffi]
// fn perform_half_miracles_in_other_direction(input: ffi::String) -> Wire<'static, MyWiredType> {
//     let value = MyWiredType { name: input.as_str().to_owned(), values: vec![] };
//     let mut out = Wire::with_size(value.live_size());
//     out.serialize(&value).unwrap();
//     out
// }
