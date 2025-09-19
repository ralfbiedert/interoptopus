// struct Packed1;
// struct Packed2;
//
// #[ffi_function]
// pub fn alignment_1(a: Packed1) -> Packed2 {
//     Packed2 { x: a.x, y: a.y }
// }
//
// #[ffi_function]
// pub fn behavior_panics_via_result() -> ffi::Result<(), Error> {
//     panic_to_result(|| result_to_ffi(|| Ok(())))
// }
//
// /// Blah
// ///
// /// Foo
// #[ffi_function]
// pub fn generic_1c<'a>(_x: Option<&'a Generic<'a, u8>>, y: &Generic<'a, u8>) -> u8 {
//     *y.x
// }
//
// /// # Safety
// ///
// /// Parameter x must point to valid data.
// #[ffi_function]
// #[allow(unused_unsafe)]
// pub unsafe fn ptr3(x: *mut i64) -> *mut i64 {
//     unsafe { *x = -*x };
//     x
// }
//
// #[ffi_function(debug)]
// pub fn ref5(x: &mut EnumPayload) {
//     *x = EnumPayload::C(123);
// }
