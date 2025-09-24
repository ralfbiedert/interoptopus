use crate::patterns::result::Error;
use crate::types::string::UseString;
use interoptopus::ffi;
use interoptopus::pattern::asynk::{Async, AsyncRuntime};
use interoptopus::pattern::result::result_to_ffi;
use tokio::runtime::{Builder, Runtime};

#[ffi(service)]
pub struct ServiceAsyncVecString {
    runtime: Runtime,
}

#[ffi]
impl ServiceAsyncVecString {
    pub fn new() -> ffi::Result<Self, Error> {
        result_to_ffi(|| {
            let runtime = Builder::new_multi_thread().build().map_err(|_| Error::Fail)?;
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

impl AsyncRuntime for ServiceAsyncVecString {
    type T = ();

    fn spawn<Fn, F>(&self, f: Fn)
    where
        Fn: FnOnce(()) -> F,
        F: Future<Output = ()> + Send + 'static,
    {
        self.runtime.spawn(f(()));
    }
}
