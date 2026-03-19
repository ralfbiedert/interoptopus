use interoptopus::{ffi, service};

#[ffi]
pub enum Error {
    A,
}

#[ffi(service)]
pub struct ServiceFoo {}

#[ffi]
impl ServiceFoo {
    pub fn create() -> ffi::Result<Self, Error> {
        ffi::Ok(Self {})
    }
}

#[test]
fn foo() {
    test_output!("Interop.cs", [service!(ServiceFoo)]);
}
