use interoptopus::ffi_type;

#[ffi_type(opaque)]
struct Opaque {
    x: std::sync::Mutex<u32>,
}

fn main() {}
