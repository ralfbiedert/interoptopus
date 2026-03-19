use interoptopus::{ffi, function};

#[ffi(export = unique)]
pub fn sum_slice(values: ffi::Slice<u32>) -> u32 {
    values.iter().sum()
}

#[test]
fn basic() {
    test_output!("Interop.cs", [function!(sum_slice)]);
}
