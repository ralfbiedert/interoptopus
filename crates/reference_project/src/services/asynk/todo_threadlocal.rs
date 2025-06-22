use crate::patterns::result::Error;
use interoptopus::ffi;
use interoptopus::pattern::asynk::{AsyncRuntimeThreadLocal, AsyncThreadLocal};
use interoptopus::pattern::result::result_to_ffi;
use interoptopus::{ffi_service, ffi_type};
use tokio::runtime::{Builder, Runtime};

type This = AsyncThreadLocal<ServiceAsyncThreadLocal, ThreadLocal>;

pub struct ThreadLocal {
    _x: u32,
}

#[ffi_type(opaque)]
pub struct ServiceAsyncThreadLocal {
    runtime: Runtime,
}

#[ffi_service]
impl ServiceAsyncThreadLocal {
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

impl AsyncRuntimeThreadLocal for ServiceAsyncThreadLocal {
    type ThreadLocal = ThreadLocal;

    fn spawn<Fn, F>(&self, _: Fn)
    where
        Fn: FnOnce(Self::ThreadLocal) -> F + Send + 'static,
        F: Future<Output = ()> + 'static,
    {
        // TODO: Run this on another runtime that supports !Send / thread-per-core futures
        // self.runtime.spawn(f());
    }
}
