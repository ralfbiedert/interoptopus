use crate::types::enums::EnumPayload;
use interoptopus::{ffi, ffi_function};

#[ffi_function]
pub fn ref1(x: &i64) -> &i64 {
    x
}

#[ffi_function]
pub fn ref2(x: &mut i64) -> &mut i64 {
    *x = -*x;
    x
}

#[ffi_function]
pub fn ref3(x: Option<&i64>) -> bool {
    x.is_some()
}

#[ffi_function]
pub fn ref4(x: Option<&mut i64>) -> bool {
    x.is_some()
}

#[ffi_function]
pub fn ref5(x: &mut EnumPayload) {
    *x = EnumPayload::C(123);
}

#[ffi_function]
pub fn ref6(x: &mut ffi::Option<EnumPayload>) {
    *x = ffi::Some(EnumPayload::C(123));
}

#[ffi_function]
pub fn ref7(x: &mut ffi::Vec<ffi::String>) {
    *x = ffi::Vec::from_vec(vec!["1".to_string().into(), "2".to_string().into(), "3".to_string().into()]);
}
