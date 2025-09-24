use crate::types::basic::Tupled;
use interoptopus::ffi;

#[ffi(transparent)]
#[allow(dead_code)]
pub struct Transparent(Tupled);
