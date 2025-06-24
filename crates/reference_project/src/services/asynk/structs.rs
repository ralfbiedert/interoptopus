use crate::patterns::result::Error;
use crate::types::arrays::NestedArray;
use interoptopus::ffi;
use interoptopus::pattern::asynk::{AsyncRuntime, AsyncSelf};
use interoptopus::pattern::result::result_to_ffi;
use interoptopus::{ffi_service, ffi_type};
use tokio::runtime::{Builder, Runtime};

#[ffi_type(opaque)]
pub struct ServiceAsyncStructs {
    runtime: Runtime,
}

#[ffi_service]
impl ServiceAsyncStructs {
    pub fn new() -> ffi::Result<Self, Error> {
        result_to_ffi(|| {
            let runtime = Builder::new_multi_thread().build().map_err(|_| Error::Fail)?;
            Ok(Self { runtime })
        })
    }

    pub async fn process_struct(_: AsyncSelf<Self>, mut x: NestedArray) -> ffi::Result<NestedArray, Error> {
        x.field_int += 1;
        ffi::Result::Ok(x)
    }
}

impl AsyncRuntime for ServiceAsyncStructs {
    fn spawn<Fn, F>(&self, f: Fn)
    where
        Fn: FnOnce(()) -> F,
        F: Future<Output = ()> + Send + 'static,
    {
        self.runtime.spawn(f(()));
    }
}
