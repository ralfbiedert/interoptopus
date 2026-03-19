use interoptopus::{extra_type, ffi};

#[ffi]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[test]
fn basic() {
    test_output!("Interop.cs", [extra_type!(Color)]);
}
