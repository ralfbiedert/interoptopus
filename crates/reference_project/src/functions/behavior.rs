use crate::patterns::result::Error;
use interoptopus::pattern::result::{panic_to_result, result_to_ffi};
use interoptopus::{ffi, ffi_function};
use std::time::Duration;

#[ffi_function]
#[allow(unreachable_code)]
pub fn behavior_panics() {
    panic!("Oh no");
}

#[ffi_function]
#[allow(unreachable_code)]
pub fn behavior_panics_via_result() -> ffi::Result<(), Error> {
    panic_to_result(|| {
        panic!("Oh no");
        result_to_ffi(|| Ok(()))
    })
}

#[ffi_function]
pub fn behavior_sleep(millis: u64) {
    std::thread::sleep(Duration::from_millis(millis));
}
