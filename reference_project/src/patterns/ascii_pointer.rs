use crate::types::UseAsciiStringPattern;
use interoptopus::ffi_function;
use interoptopus::patterns::string::AsciiPointer;

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_ascii_pointer_1(x: AsciiPointer) -> u32 {
    x.as_c_str().map(|x| x.to_bytes().len()).unwrap_or(0) as u32
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_ascii_pointer_len(x: AsciiPointer, y: UseAsciiStringPattern) -> u32 {
    let x1 = x.as_str().map(|x| x.len()).unwrap_or(0);
    let x2 = y.ascii_string.as_str().map(|x| x.len()).unwrap_or(0);
    (x1 + x2) as u32
}
