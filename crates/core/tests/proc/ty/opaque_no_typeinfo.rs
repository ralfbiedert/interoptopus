use interoptopus::ffi;

#[ffi(opaque)]
struct Opaque {
    x: std::sync::Mutex<u32>,
}

fn main() {}
