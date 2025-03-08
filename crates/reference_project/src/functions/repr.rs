use crate::types::Transparent;
use interoptopus::ffi_function;

#[ffi_function]
pub fn repr1(x: Transparent, _r: &Transparent) -> Transparent {
    x
}
