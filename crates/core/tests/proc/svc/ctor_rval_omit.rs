use interoptopus::ffi;

#[ffi]
enum Error {
    Something,
}

#[ffi(service)]
struct Service;

#[ffi]
impl Service {
    pub fn new() {}
}

fn main() {}
