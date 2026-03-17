use crate::patterns::result::Error;
use interoptopus::ffi;
use interoptopus::pattern::asynk::Async;

use super::basic::ServiceAsyncBasic;

/// A service whose construction is async.
///
/// Instead of embedding its own runtime, it borrows another
/// service that implements `AsyncRuntime` to spawn the
/// construction work.
#[ffi(service)]
pub struct ServiceAsyncCtor {
    val: u32,
}

#[ffi]
impl ServiceAsyncCtor {
    /// Async constructor that receives a runtime from the caller.
    pub async fn new_async(_runtime: Async<ServiceAsyncBasic>, x: u32) -> ffi::Result<Self, Error> {
        ffi::Ok(Self { val: x })
    }

    pub fn get_value(&self) -> u32 {
        self.val
    }
}
