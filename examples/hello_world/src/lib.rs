use interoptopus::{ffi_function, ffi_type};

/// A simple type in our FFI layer.
#[ffi_type]
#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

/// Function using the type.
#[ffi_function]
#[no_mangle]
pub extern "C" fn my_function(input: Vec2) -> Vec2 {
    input
}

// This will create a function `my_inventory` which can produce
// an abstract FFI representation (called `Library`) for this crate.
interoptopus::inventory!(my_inventory, [], [my_function], [], []);
