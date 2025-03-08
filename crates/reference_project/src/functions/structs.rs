use crate::patterns::result::FFIError;
use crate::types::{BoolField, Tupled, Vec3f32};
use interoptopus::ffi_function;
use interoptopus::patterns::result::FFIResult;

#[ffi_function]
pub fn struct1(x: Tupled) -> Tupled {
    Tupled(x.0 * 2)
}

#[ffi_function]
pub fn struct2(_a: Vec3f32, _b: Option<&Tupled>) -> FFIResult<(), FFIError> {
    FFIResult::ok(())
}

#[ffi_function]
pub fn struct3(x: BoolField) -> bool {
    x.val
}
