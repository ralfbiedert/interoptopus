//! Various ways to define constants.

use interoptopus::ffi_constant;

const fn f(x: i32) -> i32 {
    -x
}

#[ffi_constant]
pub const U8: u8 = u8::MAX;

#[ffi_constant]
pub const F32_MIN_POSITIVE: f32 = f32::MIN_POSITIVE;

#[ffi_constant]
pub const COMPUTED_I32: i32 = f(i32::MAX);
