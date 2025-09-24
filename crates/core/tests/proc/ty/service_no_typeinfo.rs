use interoptopus::ffi;

#[ffi(service)]
struct Service {
    x: std::sync::Mutex<u32>,
}

fn main() {}
