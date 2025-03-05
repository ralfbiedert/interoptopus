use crate::patterns::result::Error;
use crate::patterns::result::FFIError;
use crate::types::NestedArray;
use interoptopus::patterns::asynk::{AsyncRuntime, AsyncRuntimeThreadLocal, AsyncThreadLocal};
use interoptopus::{ffi_service, ffi_service_ctor, ffi_type};
use std::future::Future;
use tokio::runtime::Runtime;

#[ffi_type(opaque)]
pub struct ServiceAsync {
    runtime: Runtime,
}

#[ffi_service(error = "FFIError", debug)]
impl ServiceAsync {
    #[ffi_service_ctor]
    pub fn new() -> Result<Self, Error> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .map_err(|_| Error::Bad)?;

        Ok(Self { runtime })
    }

    pub async fn return_after_ms(this: This, x: u64, ms: u64) -> Result<u64, FFIError> {
        tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
        Ok(x)
    }

    pub async fn process_struct(this: This, mut x: NestedArray) -> Result<NestedArray, FFIError> {
        x.field_int += 1;
        Ok(x)
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
