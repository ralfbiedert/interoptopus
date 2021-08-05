use interoptopus::{callback, ffi_function};

callback!(MyCallback(x: u32) -> u32);

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_callback_1(callback: MyCallback, x: u32) -> u32 {
    callback.call(x)
}

// TODO?
// #[ffi_callback]
// struct VideoDataCallbackXX(fn(descriptors: &VideoFrameDescriptor, data: FFISlice<u8>));
//
// #[ffi_callback]
// type X = Callback<fn(descriptors: &VideoFrameDescriptor, data: FFISlice<u8>)>;
