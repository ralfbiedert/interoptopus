use interoptopus::ffi;

#[ffi]
enum Error {
    Something,
}

#[ffi(service)]
struct Service;

#[ffi]
impl Service {
    pub fn new() -> ffi::Result<u8, Error> {
        ffi::Ok(0)
    }
}

fn main() {}
