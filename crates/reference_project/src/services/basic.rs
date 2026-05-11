use interoptopus::ffi;

#[ffi(service)]
pub struct ServiceBasic {}

#[ffi]
impl ServiceBasic {
    pub fn create() -> Self {
        Self {}
    }
}
