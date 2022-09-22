use interoptopus::ffi_function;
use interoptopus::patterns::array_ptr::ArrayPointer;

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_array_pointer_last_or_default(ptr: ArrayPointer<u32>, len: u64, default_value: u32) -> u32 {
    unsafe { ptr.as_slice(len).last().cloned().unwrap_or(default_value) }
}
