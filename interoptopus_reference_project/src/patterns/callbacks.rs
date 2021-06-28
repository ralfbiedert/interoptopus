use interoptopus::{ffi_function, pattern_callback};

pattern_callback!(MyCallback(x: u32) -> u32);

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_callback_1(callback: MyCallback, x: u32) -> u32 {
    callback.call(x)
}
