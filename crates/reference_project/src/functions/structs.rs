use crate::patterns::result::Error;
use crate::types::{BoolField, Tupled, Vec3f32};
use interoptopus::ffi_function;
use interoptopus::pattern::result::Result;

#[ffi_function]
pub fn struct1(x: Tupled) -> Tupled {
    Tupled(x.0 * 2)
}

#[ffi_function]
pub fn struct2(_a: Vec3f32, _b: Option<&Tupled>) -> Result<(), Error> {
    Result::ok(())
}

#[ffi_function]
pub fn struct3(x: BoolField) -> bool {
    x.val
}
