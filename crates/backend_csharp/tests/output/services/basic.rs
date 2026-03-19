use interoptopus::{ffi, service};

#[ffi]
pub enum Error {
    General,
}

#[ffi(service)]
pub struct Counter {
    count: u32,
}

#[ffi(export = unique)]
impl Counter {
    pub fn create() -> ffi::Result<Self, Error> {
        ffi::Ok(Self { count: 0 })
    }

    pub fn get_count(&self) -> u32 {
        self.count
    }
}

#[test]
fn basic() {
    test_output!("Interop.cs", [service!(Counter)]);
}
