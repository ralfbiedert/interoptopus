use crate::types::CallbackFFISlice;
use interoptopus::ffi_function;
use interoptopus::patterns::slice::FFISlice;

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_ffi_slice(ffi_slice: FFISlice<u32>) -> u32 {
    ffi_slice.as_slice().unwrap_or(&[]).len() as u32
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_ffi_slice_delegate(callback: CallbackFFISlice) -> u8 {
    callback.call(FFISlice::from_slice(&[1, 2, 3]))
}
