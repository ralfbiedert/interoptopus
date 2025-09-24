use crate::types::aligned::{Packed1, Packed2};
use interoptopus::ffi;

#[ffi]
pub fn alignment_1(a: Packed1) -> Packed2 {
    Packed2 { x: a.x, y: a.y }
}
