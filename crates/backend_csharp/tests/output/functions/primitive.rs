use interoptopus::{ffi, function};

#[ffi(export = unique)]
pub fn sum_u32(a: u32, b: u32) -> u32 {
    a + b
}

#[test]
fn basic() {
    test_output!("Interop.cs", [function!(sum_u32)]);
}
