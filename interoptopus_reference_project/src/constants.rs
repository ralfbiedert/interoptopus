use interoptopus::ffi_constant;

const fn f(x: i32) -> i32 {
    -x
}

#[ffi_constant]
pub const C1: u8 = 1;

#[ffi_constant]
pub const C2: f32 = 1.0;

#[ffi_constant]
pub const C3: i32 = f(100);
