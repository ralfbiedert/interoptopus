use crate::patterns::result::Error;
use interoptopus::ffi;
use interoptopus::pattern::asynk::{AsyncRuntimeThreadLocal, AsyncThreadLocal};
use interoptopus::pattern::result::result_to_ffi;
use interoptopus::{ffi_service, ffi_type};
use tokio::runtime::{Builder, Runtime};

type This = AsyncThreadLocal<ServiceAsyncBasic, ThreadLocal>;

pub struct ThreadLocal {
    x: u32,
}

#[ffi_type(opaque)]
pub struct ServiceAsyncBasic {
    runtime: Runtime,
}

#[ffi_service]
impl ServiceAsyncBasic {
    pub fn new() -> ffi::Result<Self, Error> {
        result_to_ffi(|| {
            let runtime = Builder::new_multi_thread().build().map_err(|_| Error::Fail)?;
            Ok(Self { runtime })
        })
    }

    pub async fn call(_this: This) -> ffi::Result<(), Error> {
        ffi::Ok(())
    }
}

impl AsyncRuntimeThreadLocal for ServiceAsyncBasic {
    type ThreadLocal = ThreadLocal;

    fn spawn<Fn, F>(&self, f: Fn)
    where
        Fn: FnOnce(Self::ThreadLocal) -> F + Send + 'static,
        F: Future<Output = ()> + 'static,
    {
        // TODO: Run this on another runtime that supports !Send / thread-per-core futures
        // self.runtime.spawn(f());
    }
}
