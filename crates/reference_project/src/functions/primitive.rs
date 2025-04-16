use interoptopus::ffi_function;

#[ffi_function]
pub fn primitive_void() {}

#[ffi_function]
#[allow(clippy::unused_unit)]
pub fn primitive_void2() -> () {}

#[ffi_function]
pub fn primitive_bool(x: bool) -> bool {
    !x
}

#[ffi_function]
pub fn primitive_u8(x: u8) -> u8 {
    u8::MAX - x
}

#[ffi_function]
pub fn primitive_u16(x: u16) -> u16 {
    u16::MAX - x
}

#[ffi_function]
pub fn primitive_u32(x: u32) -> u32 {
    u32::MAX - x
}

#[ffi_function]
pub fn primitive_u64(x: u64) -> u64 {
    u64::MAX - x
}

#[ffi_function]
pub fn primitive_i8(x: i8) -> i8 {
    -x
}

#[ffi_function]
pub fn primitive_i16(x: i16) -> i16 {
    -x
}

#[ffi_function]
pub fn primitive_i32(x: i32) -> i32 {
    -x
}

#[ffi_function]
pub fn primitive_i64(x: i64) -> i64 {
    -x
}

#[ffi_function]
pub fn primitive_f32(x: f32) -> f32 {
    -x
}

#[ffi_function]
pub fn primitive_f64(x: f64) -> f64 {
    -x
}

#[ffi_function]
pub fn primitive_usize(x: usize) -> usize {
    usize::MAX - x
}

#[ffi_function]
pub fn primitive_isize(x: isize) -> isize {
    -x
}

#[ffi_function]
pub fn primitive_args_5(x0: i64, x1: i64, x2: i64, x3: i64, x4: i64) -> i64 {
    x0 + x1 + x2 + x3 + x4
}

#[ffi_function]
pub fn primitive_args_10(x0: i64, x1: i64, x2: i64, x3: i64, x4: i64, x5: i64, x6: i64, x7: i64, x8: i64, x9: i64) -> i64 {
    x0 + x1 + x2 + x3 + x4 + x5 + x6 + x7 + x8 + x9
}
