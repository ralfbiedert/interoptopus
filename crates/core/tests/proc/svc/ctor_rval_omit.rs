use interoptopus::{ffi_service, ffi_type};

#[ffi_type]
enum Error {
    Something,
}

#[ffi_type(service)]
struct Service;

#[ffi_service]
impl Service {
    pub fn new() {}
}

#[allow(dead_code)]
fn main() {}
