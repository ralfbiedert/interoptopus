use interoptopus::ffi_function;
use std::time::Duration;

#[ffi_function]
#[allow(unreachable_code)]
pub fn behavior_panics() {
    panic!("Oh no");
}

#[ffi_function]
pub fn behavior_sleep(millis: u64) {
    std::thread::sleep(Duration::from_millis(millis));
}
