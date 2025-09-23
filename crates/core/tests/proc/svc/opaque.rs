use interoptopus::{ffi, ffi_service, ffi_type};

#[ffi_type]
enum Error {
    Something,
}

#[ffi_type(opaque)]
struct Service;

#[ffi_service]
impl Service {
    pub fn new() -> ffi::Result<Self, Error> {
        ffi::Ok(Self)
    }
}

fn main() {}
