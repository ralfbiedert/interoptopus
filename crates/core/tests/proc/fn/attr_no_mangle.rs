use interoptopus::ffi_function;

#[ffi_function]
#[unsafe(no_mangle)]
fn foo(_: u8) {}

fn main() {}
