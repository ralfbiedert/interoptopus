use interoptopus::{extra_type, ffi};

#[ffi]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[test]
fn basic() {
    test_output!("Interop.cs", [extra_type!(Vec2)]);
}
