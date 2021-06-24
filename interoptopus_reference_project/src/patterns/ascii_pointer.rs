use crate::types::UseAsciiStringPattern;
use interoptopus::ffi_function;
use interoptopus::patterns::ascii_pointer::AsciiPointer;

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_ascii_pointer(x: AsciiPointer, y: UseAsciiStringPattern) -> u32 {
    let x1 = x.as_str().map(|x| x.len()).unwrap_or(0);
    let x2 = y.ascii_string.as_str().map(|x| x.len()).unwrap_or(0);
    (x1 + x2) as u32
}
