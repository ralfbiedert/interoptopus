use crate::patterns::result::Error;
use crate::patterns::result::FFIError;
use crate::types::{NestedArray, UseUtf8String};
use interoptopus::lang::rust::CTypeInfo;
use interoptopus::patterns::asynk::{AsyncRuntime, AsyncThreadLocal};
use interoptopus::patterns::result::FFIResult;
use interoptopus::patterns::string::Utf8String;
use interoptopus::{ffi_service, ffi_type};
use std::future::Future;
use tokio::runtime::Runtime;

#[ffi_type(opaque)]
pub struct ServiceAsync {
    runtime: Runtime,
}

fn f<T: CTypeInfo, E: CTypeInfo + interoptopus::patterns::result::FFIError>(f: impl FnOnce() -> Result<T, E>) -> FFIResult<T, E> {
    match f() {
        Ok(x) => FFIResult::ok(x),
        Err(e) => FFIResult::error(e),
    }
}

async fn f_async<T: CTypeInfo, E: CTypeInfo + interoptopus::patterns::result::FFIError>(ff: impl AsyncFnOnce() -> Result<T, E>) -> FFIResult<T, E> {
    match ff().await {
        Ok(x) => FFIResult::ok(x),
        Err(e) => FFIResult::error(e),
    }
}

#[ffi_service]
impl ServiceAsync {
    pub fn new() -> FFIResult<Self, FFIError> {
        // This is a workaround for the fact that tokio::runtime::Builder::new_multi_thread()
        // cannot be used in a const context.
        // See
        f(|| {
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .worker_threads(1)
                .enable_all()
                .build()
                .map_err(|_| Error::Bad)?;

            Ok(Self { runtime })
        })
    }

    pub async fn return_after_ms(_this: This, x: u64, ms: u64) -> FFIResult<u64, FFIError> {
        f_async(async || {
            // tokio::fs::read("x.text").await?;
            tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
            Ok(x)
        })
        .await
    }

    pub async fn process_struct(_this: This, mut x: NestedArray) -> FFIResult<NestedArray, FFIError> {
        x.field_int += 1;
        FFIResult::ok(x)
    }

    pub async fn handle_string(_this: This, s: Utf8String) -> FFIResult<Utf8String, FFIError> {
        FFIResult::ok(s)
    }

    pub async fn handle_nested_string(_this: This, s: Utf8String) -> FFIResult<UseUtf8String, FFIError> {
        FFIResult::ok(UseUtf8String { s1: s.clone(), s2: s.clone() })
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
