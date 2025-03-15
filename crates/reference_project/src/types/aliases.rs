use crate::types::arrays::CharArray;

pub type FnPtru8u8 = extern "C" fn(u8) -> u8;
pub type FnPtrCharArray = extern "C" fn(CharArray);
