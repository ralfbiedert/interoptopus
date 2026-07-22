use interoptopus::ffi;

#[ffi]
enum Error {
    Something,
}

#[ffi(service)]
struct Service {
    current: u32,
}

#[ffi]
impl Service {
    pub fn create() -> ffi::Result<Self, Error> {
        ffi::Ok(Self { current: 0 })
    }

    pub fn update(self: &mut Self, next: u32) {
        self.current = next;
    }
}

fn main() {}
