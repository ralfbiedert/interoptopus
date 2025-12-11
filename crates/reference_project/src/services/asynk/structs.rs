use crate::patterns::result::Error;
use crate::types::arrays::NestedArray;
use interoptopus::ffi;
use interoptopus::AsyncRuntime;
use interoptopus::pattern::asynk::{AsyncRuntime, Async};
use interoptopus::pattern::result::result_to_ffi;
use interoptopus::rt::Tokio;
use interoptopus::{ffi_service, ffi_type};

#[derive(AsyncRuntime)]
#[ffi_type(opaque)]
pub struct ServiceAsyncStructs {
    runtime: Tokio,
}

#[ffi_service]
impl ServiceAsyncStructs {
    pub fn new() -> ffi::Result<Self, Error> {
        result_to_ffi(|| {
            Ok(Self { runtime: Tokio::new() })
        })
    }

    pub async fn process_struct(_: Async<Self>, mut x: NestedArray) -> ffi::Result<NestedArray, Error> {
        x.field_int += 1;
        ffi::Result::Ok(x)
    }
}