use interoptopus::ffi;

#[ffi]
enum Error {
    Something,
}

#[ffi(opaque)]
struct Service;

#[ffi]
impl Service {
    pub fn create() -> ffi::Result<Self, Error> {
        ffi::Ok(Self)
    }
}

fn main() {}
