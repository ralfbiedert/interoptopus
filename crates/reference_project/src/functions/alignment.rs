use crate::types::aligned::{Packed1, Packed2};
use interoptopus::ffi_function;

#[ffi_function]
pub fn alignment_1(a: Packed1) -> Packed2 {
    Packed2 { x: a.x, y: a.y }
}
