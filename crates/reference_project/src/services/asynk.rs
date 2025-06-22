use crate::patterns::callback::StringCallback;
use crate::patterns::result::Error;
use crate::types::arrays::NestedArray;
use crate::types::string::UseString;
use interoptopus::ffi_type;
use interoptopus::pattern::asynk::{AsyncRuntime, AsyncThreadLocal};
use interoptopus::pattern::result::{result_to_ffi, result_to_ffi_async};
use interoptopus::{ffi, ffi_service};
use std::future::Future;
use tokio::runtime::Runtime;

#[ffi_type(opaque)]
pub struct ServiceAsync {
    runtime: Runtime,
}

#[ffi_service]
impl ServiceAsync {
    pub fn new() -> ffi::Result<Self, Error> {
        result_to_ffi(|| {
            // This is a workaround for the fact that tokio::runtime::Builder::new_multi_thread()
            // cannot be used in a const context.
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .worker_threads(1)
                .enable_all()
                .build()
                .map_err(|_| Error::Fail)?;

            Ok(Self { runtime })
        })
    }

    pub async fn process_struct(_this: This, mut x: NestedArray) -> ffi::Result<NestedArray, Error> {
        x.field_int += 1;
        ffi::Result::Ok(x)
    }

    pub fn callback_string(&self, s: ffi::String, cb: StringCallback) {
        cb.call(s.clone());
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
