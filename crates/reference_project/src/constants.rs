//! Various ways to define constants.

use interoptopus::ffi;

const fn f(x: i32) -> i32 {
    -x
}

#[ffi]
pub const U8: u8 = u8::MAX;

#[ffi]
pub const F32_MIN_POSITIVE: f32 = f32::MIN_POSITIVE;

#[ffi]
pub const COMPUTED_I32: i32 = f(i32::MAX);
