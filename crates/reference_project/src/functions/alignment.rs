use crate::types::aligned::{Packed1, Packed2};
use interoptopus::ffi_function;

#[ffi_function]
pub fn alignment_1(a: Packed1) -> Packed2 {
    Packed2 { x: a.x, y: a.y }
}

// UNSUPPORTED FOR NOW - Unclear how to handle BooleanAlignment in C# with LibraryImport
// #[ffi_function]
// pub fn boolean_alignment(mut x: BooleanAlignment) -> BooleanAlignment {
//     x.is_valid = !x.is_valid;
//     x
// }
//
// #[ffi_function]
// pub fn boolean_alignment2(rval: bool) -> BooleanAlignment {
//     BooleanAlignment {
//         is_valid: rval,
//         ..Default::default()
//     }
// }

// #[ffi_function]
// pub fn aligned_to_packed1(a: Aligned1) -> Packed1 {
//     dbg!(a.x);
//     dbg!(a.y);
//     Packed1 { x: a.x, y: a.y }
// }
//
// #[ffi_function(debug)]
// pub fn aligned_to_packed2(a: Aligned2) -> Packed2 {
//     Packed2 { x: a.x, y: a.y }
// }
