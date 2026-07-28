use interoptopus::{ffi, function};

#[ffi]
#[derive(Clone)]
pub struct Attribute<'a> {
    pub bytes: ffi::Slice<'a, u8>,
}

#[ffi(export = unique)]
pub fn sum_slice(values: ffi::Slice<u32>) -> u32 {
    values.iter().sum()
}

#[ffi(export = unique)]
pub fn count_attributes(values: ffi::Slice<Attribute>) -> u32 {
    values.len() as u32
}

#[test]
fn basic() {
    test_output!("Interop.cs", [function!(sum_slice)]);
}

#[test]
fn non_blittable() {
    test_output!("Interop.cs", [function!(count_attributes)]);
}
