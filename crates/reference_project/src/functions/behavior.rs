use crate::patterns::result::Error;
use interoptopus::ffi;
use interoptopus::pattern::result::{panic_to_result, result_to_ffi};
use std::time::Duration;

#[ffi]
#[allow(unreachable_code)]
pub fn behavior_panics() {
    panic!("Oh no");
}

#[ffi]
pub fn behavior_panics_via_result() -> ffi::Result<(), Error> {
    #[allow(unreachable_code)]
    panic_to_result(|| {
        panic!("Oh no");
        result_to_ffi(|| Ok(()))
    })
}

#[ffi]
pub fn behavior_sleep(millis: u64) {
    std::thread::sleep(Duration::from_millis(millis));
}
