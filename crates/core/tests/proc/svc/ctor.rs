use interoptopus::{ffi, ffi_service, ffi_type};

#[ffi_type]
enum Error {
    Something,
}

#[ffi_type(service)]
struct Service;

#[ffi_service]
impl Service {
    pub fn new() -> ffi::Result<Self, Error> {
        ffi::Ok(Self)
    }
}

#[allow(dead_code)]
fn main() {}
