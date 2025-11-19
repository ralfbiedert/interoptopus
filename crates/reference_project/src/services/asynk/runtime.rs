use crate::patterns::result::Error;
use interoptopus::ffi;
use interoptopus::pattern::asynk::{Async, AsyncRuntime};
use interoptopus::pattern::result::result_to_ffi;
use std::future::Future;
use tokio::runtime::{Builder, Runtime};
use crate::services::asynk::basic::ServiceAsyncBasic;

mod rt {
    use interoptopus::pattern::asynk::AsyncRuntime;
    use std::future::Future;

    pub struct Runtime;

    impl AsyncRuntime for Runtime {
        type T = ();

        fn spawn<Fn, F>(&self, f: Fn)
        where
            Fn: FnOnce(()) -> F,
            F: Future<Output = ()> + Send + 'static,
        {
        }
    }
}

#[ffi(service)]
#[derive(AsyncRuntime)]
pub struct ServiceRuntime {
    #[runtime::forward]
    runtime: rt::Runtime,
}

#[ffi]
impl ServiceAsyncBasic {
    pub fn new() -> ffi::Result<Self, Error> {
        todo!()
    }

    pub async fn call(_: Async<Self>) -> ffi::Result<(), Error> {
        ffi::Ok(())
    }
}
