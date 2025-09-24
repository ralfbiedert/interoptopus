use interoptopus::ffi;
use crate::patterns::result::Error;
use crate::types::basic::{Tupled, Vec3f32};
use crate::types::bool::BoolField;
use interoptopus::pattern::result::Result;

#[ffi]
pub fn struct1(x: Tupled) -> Tupled {
    Tupled(x.0 * 2)
}

#[ffi]
pub fn struct2(_a: Vec3f32, _b: Option<&Tupled>) -> Result<(), Error> {
    Result::Ok(())
}

#[ffi]
pub fn struct3(x: BoolField) -> bool {
    x.val
}
