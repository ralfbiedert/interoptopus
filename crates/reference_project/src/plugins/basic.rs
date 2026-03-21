interoptopus::plugin!(Primitives {
    fn primitive_void();
    fn primitive_u8(x: u8) -> u8;
    fn primitive_u16(x: u16) -> u16;
    fn primitive_u32(x: u32) -> u32;
    fn primitive_u64(x: u64) -> u64;
    fn primitive_i8(x: i8) -> i8;
    fn primitive_i16(x: i16) -> i16;
    fn primitive_i32(x: i32) -> i32;
    fn primitive_i64(x: i64) -> i64;
    fn primitive_f32(x: f32) -> f32;
    fn primitive_f64(x: f64) -> f64;
});

interoptopus::plugin!(Primitives2 {
    fn primitive_void();
    fn primitive_u32(x: u32) -> u32;
});
