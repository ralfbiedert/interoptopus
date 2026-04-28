use crate::patterns::result::Error;
use interoptopus::pattern::asynk::Async;
use interoptopus::pattern::result::result_to_ffi;
use interoptopus::rt::Tokio;
use interoptopus::{AsyncRuntime, ffi};

#[ffi(service)]
#[derive(AsyncRuntime)]
pub struct ServiceAsyncPanic {
    runtime: Tokio,
}

#[ffi]
impl ServiceAsyncPanic {
    pub fn create() -> ffi::Result<Self, Error> {
        result_to_ffi(|| {
            let runtime = Tokio::new();
            Ok(Self { runtime })
        })
    }

    /// Async method that always panics.
    pub async fn panicking(_: Async<Self>) -> ffi::Result<(), Error> {
        panic!("intentional panic in async method");
    }

    /// Async method that succeeds normally.
    pub async fn not_panicking(_: Async<Self>) -> ffi::Result<(), Error> {
        ffi::Ok(())
    }
}
