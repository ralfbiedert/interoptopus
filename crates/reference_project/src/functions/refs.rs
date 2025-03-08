use interoptopus::ffi_function;

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
