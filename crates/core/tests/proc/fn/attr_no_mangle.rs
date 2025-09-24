use interoptopus::ffi;

#[ffi]
#[unsafe(no_mangle)]
fn foo(_: u8) {}

fn main() {}
