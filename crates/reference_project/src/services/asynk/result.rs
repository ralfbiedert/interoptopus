use crate::patterns::result::Error;
use interoptopus::ffi;
use interoptopus::pattern::asynk::{AsyncRuntime, AsyncSelf, AsyncThreadLocal};
use interoptopus::pattern::result::result_to_ffi;
use interoptopus::ffi_type;
use tokio::runtime::{Builder, Runtime};

#[ffi_type(opaque)]
pub struct ServiceAsyncResult {
    runtime: Runtime,
}

// #[ffi_service]
// impl ServiceAsyncResult {
//     pub fn new() -> ffi::Result<Self, Error> {
//         result_to_ffi(|| {
//             let runtime = Builder::new_multi_thread().build().map_err(|_| Error::Fail)?;
//             Ok(Self { runtime })
//         })
//     }
//
//     pub async fn success(_slf: AsyncThreadLocal<ServiceAsyncResult, ()>) -> ffi::Result<(), Error> {
//         ffi::Result::Ok(())
//     }
//
//     pub async fn fail(_slf: AsyncSelf<ServiceAsyncResult>) -> ffi::Result<(), Error> {
//         ffi::Result::Err(Error::Fail)
//     }
// }

impl AsyncRuntime for ServiceAsyncResult {
    fn spawn<Fn, F>(&self, f: Fn)
    where
        Fn: FnOnce(()) -> F,
        F: Future<Output = ()> + Send + 'static,
    {
        self.runtime.spawn(f(()));
    }
}

impl ServiceAsyncResult {
    pub fn new() -> ffi::Result<Self, Error> {
        result_to_ffi(|| {
            let runtime = Builder::new_multi_thread().build().map_err(|_| Error::Fail)?;
            Ok(Self { runtime })
        })
    }

    pub async fn success(_slf: AsyncThreadLocal<ServiceAsyncResult, ()>) -> ffi::Result<(), Error> {
        ffi::Result::Ok(())
    }

    pub async fn fail(_slf: AsyncSelf<ServiceAsyncResult>) -> ffi::Result<(), Error> {
        ffi::Result::Err(Error::Fail)
    }
}
#[::interoptopus::ffi_function]
#[unsafe(no_mangle)]
#[allow(unused_mut, unsafe_op_in_unsafe_fn)]
#[allow(
    clippy::needless_lifetimes,
    clippy::extra_unused_lifetimes,
    clippy::redundant_locals,
    clippy::forget_non_drop,
    clippy::useless_conversion
)]
pub extern "C" fn service_async_result_new()
-> <<ServiceAsyncResult as ::interoptopus::pattern::service::ServiceInfo>::CtorResult as ::interoptopus::pattern::result::ResultAsPtr>::AsPtr {
    let __result_result = std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| <ServiceAsyncResult>::new()));
    match __result_result {
        Ok(__res) if __res.is_ok() => {
            let __boxed = ::std::sync::Arc::new(__res.unwrap());
            let __raw = ::std::sync::Arc::into_raw(__boxed);
            <<ServiceAsyncResult as ::interoptopus::pattern::service::ServiceInfo>::CtorResult as ::interoptopus::pattern::result::ResultAsPtr>::AsPtr::Ok(__raw)
        }
        Ok(__res) => {
            let __e = __res.unwrap_err();
            ::interoptopus::ffi::log_error(|| format!("Error in ({}): {:?}", stringify!(service_basic_new), __e));
            <<ServiceAsyncResult as ::interoptopus::pattern::service::ServiceInfo>::CtorResult as ::interoptopus::pattern::result::ResultAsPtr>::AsPtr::Err(__e)
        }
        Err(__e) => {
            ::interoptopus::ffi::log_error(|| {
                format!("Panic in ({}): {}", stringify!(service_async_result_new), ::interoptopus::pattern::result::get_panic_message(__e.as_ref()))
            });
            <<ServiceAsyncResult as ::interoptopus::pattern::service::ServiceInfo>::CtorResult as ::interoptopus::pattern::result::ResultAsPtr>::AsPtr::Panic
        }
    }
}
#[::interoptopus::ffi_function]
#[unsafe(no_mangle)]
#[allow(unused_mut, unsafe_op_in_unsafe_fn)]
#[allow(
    clippy::needless_lifetimes,
    clippy::extra_unused_lifetimes,
    clippy::redundant_locals,
    clippy::forget_non_drop,
    clippy::useless_conversion
)]
pub extern "C" fn service_async_result_success(
    __context: &ServiceAsyncResult,
    __async_callback: ::interoptopus::pattern::asynk::AsyncCallback<ffi::Result<(), Error>>,
) -> <ffi::Result<(), Error> as ::interoptopus::pattern::result::ResultAsUnitT>::AsUnitT {
    let __this = __context;
    let __arc_restored = unsafe { ::std::sync::Arc::from_raw(__context) };
    let __context = ::std::sync::Arc::clone(&__arc_restored);
    let _ = ::std::sync::Arc::into_raw(__arc_restored);
    let __async_fn = async move |__tlcontext| {
        let __context = <AsyncThreadLocal<ServiceAsyncResult, ()> as ::interoptopus::pattern::asynk::AsyncProxy<_, _>>::new(__context, __tlcontext);
        let __rval = <ServiceAsyncResult>::success(__context).await.into();
        __async_callback.call(&__rval);
        ::std::mem::forget(__rval);
    };
    <ServiceAsyncResult>::spawn(__this, __async_fn);
    <ffi::Result<(), Error> as ::interoptopus::pattern::result::ResultAsUnitT>::AsUnitT::Ok(())
}
#[::interoptopus::ffi_function]
#[unsafe(no_mangle)]
#[allow(unused_mut, unsafe_op_in_unsafe_fn)]
#[allow(
    clippy::needless_lifetimes,
    clippy::extra_unused_lifetimes,
    clippy::redundant_locals,
    clippy::forget_non_drop,
    clippy::useless_conversion
)]
pub extern "C" fn service_async_result_fail(
    __context: &ServiceAsyncResult,
    __async_callback: ::interoptopus::pattern::asynk::AsyncCallback<ffi::Result<(), Error>>,
) -> <ffi::Result<(), Error> as ::interoptopus::pattern::result::ResultAsUnitT>::AsUnitT {
    let __this = __context;
    let __arc_restored = unsafe { ::std::sync::Arc::from_raw(__context) };
    let __context = ::std::sync::Arc::clone(&__arc_restored);
    let _ = ::std::sync::Arc::into_raw(__arc_restored);
    let __async_fn = async move |__tlcontext| {
        let __context = <AsyncSelf<ServiceAsyncResult> as ::interoptopus::pattern::asynk::AsyncProxy<_, _>>::new(__context, __tlcontext);
        let __rval = <ServiceAsyncResult>::fail(__context).await.into();
        __async_callback.call(&__rval);
        ::std::mem::forget(__rval);
    };
    <ServiceAsyncResult>::spawn(__this, __async_fn);
    <ffi::Result<(), Error> as ::interoptopus::pattern::result::ResultAsUnitT>::AsUnitT::Ok(())
}
#[doc = r" Destroys the given instance."]
#[doc = r""]
#[doc = r" # Safety"]
#[doc = r""]
#[doc = r" The passed parameter MUST have been created with the corresponding init function;"]
#[doc = r" passing any other value results in undefined behavior."]
#[::interoptopus::ffi_function]
#[allow(unused_mut, unsafe_op_in_unsafe_fn, unused_unsafe)]
#[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes, clippy::redundant_locals)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn service_async_result_destroy(
    __context: *const ServiceAsyncResult,
) -> <<ServiceAsyncResult as ::interoptopus::pattern::service::ServiceInfo>::CtorResult as ::interoptopus::pattern::result::ResultAsPtr>::AsPtr {
    if __context.is_null() {
        return <<ServiceAsyncResult as ::interoptopus::pattern::service::ServiceInfo>::CtorResult as ::interoptopus::pattern::result::ResultAsPtr>::AsPtr::Null;
    }
    let __result_result = ::std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        unsafe { drop(::std::sync::Arc::from_raw(__context)) };
    }));
    match __result_result {
        Ok(_) => <<ServiceAsyncResult as ::interoptopus::pattern::service::ServiceInfo>::CtorResult as ::interoptopus::pattern::result::ResultAsPtr>::AsPtr::Ok(
            ::std::ptr::null(),
        ),
        Err(__e) => {
            ::interoptopus::ffi::log_error(|| {
                format!("Panic in ({}): {}", stringify!(service_async_result_destroy), ::interoptopus::pattern::result::get_panic_message(__e.as_ref()))
            });
            <<ServiceAsyncResult as ::interoptopus::pattern::service::ServiceInfo>::CtorResult as ::interoptopus::pattern::result::ResultAsPtr>::AsPtr::Panic
        }
    }
}
impl ::interoptopus::pattern::service::ServiceInfo for ServiceAsyncResult {
    type CtorResult = ffi::Result<Self, Error>;
}
impl ::interoptopus::pattern::LibraryPatternInfo for ServiceAsyncResult {
    fn pattern_info() -> ::interoptopus::pattern::LibraryPattern {
        use ::interoptopus::lang::FunctionInfo;
        let mut methods = Vec::new();
        let mut ctors = Vec::new();
        {
            use service_async_result_success as x;
            methods.push(x::function_info());
        }
        {
            use service_async_result_fail as x;
            methods.push(x::function_info());
        }
        {
            use service_async_result_new as x;
            ctors.push(x::function_info());
        }
        let dtor = {
            use service_async_result_destroy as x;
            x::function_info()
        };
        let service = ::interoptopus::pattern::service::ServiceDefinition::new(ctors, dtor, methods);
        service.assert_valid();
        ::interoptopus::pattern::LibraryPattern::Service(service)
    }
}
