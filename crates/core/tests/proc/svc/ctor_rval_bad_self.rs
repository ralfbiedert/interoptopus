use interoptopus::{ffi, ffi_service, ffi_type};

#[ffi_type]
enum Error {
    Something,
}

#[ffi_type(service)]
struct Service;

#[ffi_service]
impl Service {
    pub fn new() -> ffi::Result<u8, Error> {
        ffi::Ok(0)
    }
}

#[allow(dead_code)]
fn main() {}
