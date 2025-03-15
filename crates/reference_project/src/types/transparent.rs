use crate::types::basic::Tupled;
use interoptopus::ffi_type;

#[ffi_type(transparent)]
#[allow(dead_code)]
pub struct Transparent(Tupled);
