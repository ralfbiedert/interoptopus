use interoptopus::ffi_type;

#[ffi_type(service)]
struct Service {
    x: std::sync::Mutex<u32>,
}

fn main() {}
