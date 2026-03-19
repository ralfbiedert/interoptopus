use interoptopus::wire::Wire;
use interoptopus::{ffi, function};

#[ffi(export = unique)]
pub fn greeting() -> Wire<String> {
    Wire::from("hello".to_string())
}

#[test]
fn basic() {
    test_output!("Interop.cs", [function!(greeting)]);
}
