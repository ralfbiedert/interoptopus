use crate::types::transparent::Transparent;
use interoptopus::ffi;

#[ffi]
pub fn repr1(x: Transparent, _r: &Transparent) -> Transparent {
    x
}
