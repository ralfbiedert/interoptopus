use crate::patterns::callback::StringCallback;
use crate::patterns::result::ErrorREMOVEME;
use crate::patterns::result::ErrorREMOVEME2;
use crate::types::arrays::NestedArray;
use crate::types::string::UseString;
use interoptopus::ffi;
use interoptopus::pattern::asynk::{AsyncRuntime, AsyncThreadLocal};
use interoptopus::pattern::result::{result_to_ffi, result_to_ffi_async};
use interoptopus::{ffi_service, ffi_type};
use std::future::Future;
use tokio::runtime::Runtime;

#[ffi_type(opaque)]
pub struct ServiceAsync {
    runtime: Runtime,
}

#[ffi_service]
impl ServiceAsync {
    pub fn new() -> ffi::Result<Self, ErrorREMOVEME> {
        result_to_ffi(|| {
            // This is a workaround for the fact that tokio::runtime::Builder::new_multi_thread()
            // cannot be used in a const context.
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .worker_threads(1)
                .enable_all()
                .build()
                .map_err(|_| ErrorREMOVEME2::Bad)?;

            Ok(Self { runtime })
        })
    }

    pub async fn return_after_ms(_this: This, x: u64, ms: u64) -> ffi::Result<u64, ErrorREMOVEME> {
        result_to_ffi_async(async || {
            // tokio::fs::read("x.text").await?;
            tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
            Ok(x)
        })
        .await
    }

    pub async fn process_struct(_this: This, mut x: NestedArray) -> ffi::Result<NestedArray, ErrorREMOVEME> {
        x.field_int += 1;
        ffi::Result::Ok(x)
    }

    pub async fn handle_string(_this: This, s: ffi::String) -> ffi::Result<ffi::String, ErrorREMOVEME> {
        ffi::Result::Ok(s)
    }

    pub async fn handle_nested_string(_this: This, s: ffi::String) -> ffi::Result<UseString, ErrorREMOVEME> {
        ffi::Result::Ok(UseString { s1: s.clone(), s2: s.clone() })
    }

    pub fn callback_string(&self, s: ffi::String, cb: StringCallback) {
        cb.call(s.clone());
    }

    pub async fn fail(_this: This) -> ffi::Result<(), ErrorREMOVEME> {
        ffi::Result::Err(ErrorREMOVEME::Fail)
    }

    // TODO: This must not compile.
    pub fn bad(&mut self) {}
}

type ThreadLocal = ();
type This = AsyncThreadLocal<ServiceAsync, ThreadLocal>;

impl AsyncRuntime for ServiceAsync {
    fn spawn<Fn, F>(&self, f: Fn)
    where
        Fn: FnOnce(()) -> F,
        F: Future<Output = ()> + Send + 'static,
    {
        self.runtime.spawn(f(()));
    }
}
