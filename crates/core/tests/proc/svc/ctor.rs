use interoptopus::ffi;

#[ffi]
enum Error {
    Something,
}

#[ffi(service)]
struct Service;

#[ffi]
impl Service {
    pub fn new() -> ffi::Result<Self, Error> {
        ffi::Ok(Self)
    }
}

fn main() {}
