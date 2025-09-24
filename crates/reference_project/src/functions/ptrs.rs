use interoptopus::ffi;

#[ffi]
pub fn ptr1(x: *const i64) -> *const i64 {
    x
}

#[ffi]
pub fn ptr2(x: *const *const i64) -> *const *const i64 {
    x
}

/// # Safety
///
/// Parameter x must point to valid data.
#[ffi]
#[allow(unused_unsafe)]
pub unsafe fn ptr3(x: *mut i64) -> *mut i64 {
    unsafe { *x = -*x };
    x
}
