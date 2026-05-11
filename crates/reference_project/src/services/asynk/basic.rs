use interoptopus::rt::Tokio;
use interoptopus::{AsyncRuntime, ffi};

#[ffi(service)]
#[derive(AsyncRuntime)]
pub struct ServiceAsyncBasic {
    runtime: Tokio,
}

#[ffi]
impl ServiceAsyncBasic {
    pub fn simple() -> Self {
        let runtime = Tokio::new();
        Self { runtime }
    }
}
