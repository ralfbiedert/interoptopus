use crate::patterns::result::Error;
use crate::types::string::UseString;
use interoptopus::pattern::asynk::Async;
use interoptopus::pattern::result::result_to_ffi;
use interoptopus::rt::Tokio;
use interoptopus::{ffi, AsyncRuntime};

#[ffi(service)]
#[derive(AsyncRuntime)]
pub struct ServiceAsyncVecString {
    #[runtime(forward)]
    runtime: Tokio,
}

#[ffi]
impl ServiceAsyncVecString {
    pub fn new() -> ffi::Result<Self, Error> {
        result_to_ffi(|| {
            let runtime = Tokio::new();
            Ok(Self { runtime })
        })
    }

    pub async fn handle_string(_: Async<Self>, s: ffi::String) -> ffi::Result<ffi::String, Error> {
        ffi::Result::Ok(s)
    }

    pub async fn handle_vec_string(_: Async<Self>, s: ffi::Vec<ffi::String>) -> ffi::Result<ffi::Vec<ffi::String>, Error> {
        ffi::Result::Ok(s)
    }

    pub async fn handle_nested_string(_: Async<Self>, s: ffi::String) -> ffi::Result<UseString, Error> {
        ffi::Result::Ok(UseString { s1: s.clone(), s2: s.clone() })
    }
}
