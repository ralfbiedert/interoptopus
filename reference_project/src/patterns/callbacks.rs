use interoptopus::{callback, ffi_function};
use std::ffi::c_void;

callback!(MyCallback(value: u32) -> u32);
callback!(MyCallbackVoid(ptr: *const c_void));

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_callback_1(callback: MyCallback, x: u32) -> u32 {
    callback.call(x)
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_callback_2(callback: MyCallbackVoid) -> MyCallbackVoid {
    callback
}
