use crate::patterns::result::Error;
use crate::types::string::UseString;
use interoptopus::ffi;
use interoptopus::pattern::asynk::{AsyncRuntime, AsyncSelf};
use interoptopus::pattern::result::result_to_ffi;
use interoptopus::{ffi_service, ffi_type};
use tokio::runtime::{Builder, Runtime};

type This = AsyncSelf<ServiceAsyncVecString>;

#[ffi_type(opaque)]
pub struct ServiceAsyncVecString {
    runtime: Runtime,
}

#[ffi_service]
impl ServiceAsyncVecString {
    pub fn new() -> ffi::Result<Self, Error> {
        result_to_ffi(|| {
            let runtime = Builder::new_multi_thread().enable_time().build().map_err(|_| Error::Fail)?;
            Ok(Self { runtime })
        })
    }

    pub async fn handle_string(_this: This, s: ffi::String) -> ffi::Result<ffi::String, Error> {
        ffi::Result::Ok(s)
    }

    pub async fn handle_vec_string(_this: This, s: ffi::Vec<ffi::String>) -> ffi::Result<ffi::Vec<ffi::String>, Error> {
        ffi::Result::Ok(s)
    }

    pub async fn handle_nested_string(_this: This, s: ffi::String) -> ffi::Result<UseString, Error> {
        ffi::Result::Ok(UseString { s1: s.clone(), s2: s.clone() })
    }
}

impl AsyncRuntime for ServiceAsyncVecString {
    fn spawn<Fn, F>(&self, f: Fn)
    where
        Fn: FnOnce(()) -> F,
        F: Future<Output = ()> + Send + 'static,
    {
        self.runtime.spawn(f(()));
    }
}
