use interoptopus::{callback, ffi_function};

callback!(MyCallback(x: u32) -> u32);
callback!(MyCallbackVoid());

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_callback_1(callback: MyCallback, x: u32) -> u32 {
    callback.call(x)
}
