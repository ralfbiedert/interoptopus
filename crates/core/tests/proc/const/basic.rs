use interoptopus::ffi;

#[ffi]
const X: u8 = 123;

#[ffi]
const Y: usize = size_of::<usize>();

fn main() {}
