use crate::patterns::result::Error;
use interoptopus::pattern::asynk::Async;
use interoptopus::pattern::result::result_to_ffi;
use interoptopus::rt::Tokio;
use interoptopus::wire::Wire;
use interoptopus::{AsyncRuntime, ffi};
use std::collections::HashMap;

#[ffi(service)]
#[derive(AsyncRuntime)]
pub struct ServiceAsyncWire {
    #[runtime]
    rt: Tokio,
}

#[ffi]
impl ServiceAsyncWire {
    pub fn create() -> ffi::Result<Self, Error> {
        result_to_ffi(|| {
            let rt = Tokio::new();
            Ok(Self { rt })
        })
    }

    pub async fn wire_passthrough(_: Async<Self>, x: Wire<HashMap<String, String>>) -> ffi::Result<Wire<HashMap<String, String>>, Error> {
        ffi::Result::Ok(x)
    }
}
