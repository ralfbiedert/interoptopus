use interoptopus::ffi_function;
use interoptopus::patterns::primitives::FFIBool;

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_ffi_bool(ffi_bool: FFIBool) -> FFIBool {
    !ffi_bool
}
