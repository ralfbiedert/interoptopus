use crate::patterns::result::Error;
use crate::types::arrays::NestedArray;
use interoptopus::ffi;
use interoptopus::pattern::asynk::{Async, AsyncRuntime};
use interoptopus::pattern::result::result_to_ffi;
use tokio::runtime::{Builder, Runtime};

#[ffi(service)]
pub struct ServiceAsyncStructs {
    runtime: Runtime,
}

#[ffi]
impl ServiceAsyncStructs {
    pub fn new() -> ffi::Result<Self, Error> {
        result_to_ffi(|| {
            let runtime = Builder::new_multi_thread().build().map_err(|_| Error::Fail)?;
            Ok(Self { runtime })
        })
    }

    pub async fn process_struct(_: Async<Self>, mut x: NestedArray) -> ffi::Result<NestedArray, Error> {
        x.field_int += 1;
        ffi::Result::Ok(x)
    }
}

impl AsyncRuntime for ServiceAsyncStructs {
    type T = ();

    fn spawn<Fn, F>(&self, f: Fn)
    where
        Fn: FnOnce(()) -> F,
        F: Future<Output = ()> + Send + 'static,
    {
        self.runtime.spawn(f(()));
    }
}
