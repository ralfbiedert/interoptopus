// use crate::patterns::result::FFIError;
// use interoptopus::patterns::asynk::AsyncCallback;
// use interoptopus::patterns::result::FFIResult;
// use interoptopus::{ffi_service_ctor, ffi_type, Error};
// use std::sync::Arc;
// use std::thread::{sleep, spawn};
//
// #[ffi_type(opaque)]
// pub struct ServiceAsync {
//     runtime: (),
// }
//
// impl ServiceAsync {
//     #[ffi_service_ctor]
//     pub fn new() -> Result<Arc<Self>, Error> {
//         Ok(Arc::new(Self { runtime: () }))
//     }
//
//     pub async fn return_after_ms2(&self, x: u64, ms: u64) -> Result<u64, FFIError> {
//         // sleep(std::time::Duration::from_millis(ms));
//         // async_callback.call(&x);
//         Ok(x)
//     }
//
//     pub fn return_after_ms(&self, x: u64, ms: u64, async_callback: AsyncCallback<FFIResult<u64, FFIError>>) -> Result<(), FFIError> {
//         spawn(move || {
//             sleep(std::time::Duration::from_millis(ms));
//             async_callback.call(&FFIResult::ok(x));
//         });
//         Ok(())
//     }
//
//     pub fn f(&mut self) -> Result<(), FFIError> {
//         Ok(())
//     }
//
//     // pub async fn do_work(&self) -> Result<u8, Error> {
//     //     Ok(123)
//     // }
//
//     // pub fn do_work_raw(&self, callback: CallbackU8) -> FFIError {
//     //     self.runtime.spawn(|| async {
//     //         let rval = self.do_work().await;
//     //         callback.call(rval);
//     //     })
//     // }
// }
// #[::interoptopus::ffi_function]
// #[no_mangle]
// #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
// #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes, clippy::redundant_locals)]
// pub extern "C" fn service_async_new(context: &mut *const ServiceAsync) -> FFIError {
//     *context = ::std::ptr::null_mut();
//     let result_result = std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| <ServiceAsync>::new()));
//     match result_result {
//         Ok(Ok(obj)) => {
//             let boxed = Arc::new(obj);
//             let raw = Arc::into_raw(boxed);
//             *context = raw;
//             <FFIError as ::interoptopus::patterns::result::FFIError>::SUCCESS
//         }
//         Ok(Err(e)) => {
//             ::interoptopus::util::log_error(|| format!("Error in ({}): {:?}", stringify!(service_async_new), e));
//             e.into()
//         }
//         Err(e) => {
//             ::interoptopus::util::log_error(|| {
//                 format!(
//                     "Panic in ({}): {}",
//                     stringify!(service_async_new),
//                     ::interoptopus::patterns::result::get_panic_message(e.as_ref())
//                 )
//             });
//             <FFIError as ::interoptopus::patterns::result::FFIError>::PANIC
//         }
//     }
// }
//
// #[::interoptopus::ffi_function]
// #[no_mangle]
// #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
// #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes, clippy::redundant_locals)]
// pub extern "C" fn service_async_return_after_ms2(context: &ServiceAsync, x: u64, ms: u64, async_callback: AsyncCallback<FFIResult<u64, FFIError>>) -> FFIError {
//     use ::interoptopus::patterns::result::FFIError;
//     let f2 = <ServiceAsync>::return_after_ms2(
//         // sleep(std::time::Duration::from_millis(ms));
//         // async_callback.call(&x);
//         context, x, ms,
//     );
//     let f1 = async move {
//         // sleep(std::time::Duration::from_millis(ms));
//         // async_callback.call(&x);
//         f2.await;
//     };
//     // <ServiceAsync>::spawn(
//     //     // sleep(std::time::Duration::from_millis(ms));
//     //     // async_callback.call(&x);
//     //     context, f1,
//     // );
//     FFIError::SUCCESS
// }
//
// #[::interoptopus::ffi_function]
// #[no_mangle]
// #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
// #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes, clippy::redundant_locals)]
// pub extern "C" fn service_async_return_after_ms(context: &ServiceAsync, x: u64, ms: u64, async_callback: AsyncCallback<FFIResult<u64, FFIError>>) -> FFIError {
//     ::interoptopus::patterns::result::panics_and_errors_to_ffi_enum(
//         move || {
//             <ServiceAsync>::return_after_ms(context, x, ms, async_callback)
//
//             // pub async fn do_work(&self) -> Result<u8, Error> {
//             //     Ok(123)
//             // }
//
//             // pub fn do_work_raw(&self, callback: CallbackU8) -> FFIError {
//             //     self.runtime.spawn(|| async {
//             //         let rval = self.do_work().await;
//             //         callback.call(rval);
//             //     })
//             // }
//         },
//         stringify!(service_async_return_after_ms),
//     )
// }
//
//
// #[::interoptopus::ffi_function]
// #[no_mangle]
// #[allow(unused_mut, unsafe_op_in_unsafe_fn)]
// #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes, clippy::redundant_locals)]
// pub extern "C" fn service_async_return_f(context: &ServiceAsync) -> FFIError {
//     ::interoptopus::patterns::result::panics_and_errors_to_ffi_enum(
//         move || {
//             <ServiceAsync>::return_after_ms(context, x, ms, async_callback)
//
//             // pub async fn do_work(&self) -> Result<u8, Error> {
//             //     Ok(123)
//             // }
//
//             // pub fn do_work_raw(&self, callback: CallbackU8) -> FFIError {
//             //     self.runtime.spawn(|| async {
//             //         let rval = self.do_work().await;
//             //         callback.call(rval);
//             //     })
//             // }
//         },
//         stringify!(service_async_return_after_ms),
//     )
// }
//
//
// #[doc = r" Destroys the given instance."]
// #[doc = r""]
// #[doc = r" # Safety"]
// #[doc = r""]
// #[doc = r" The passed parameter MUST have been created with the corresponding init function;"]
// #[doc = r" passing any other value results in undefined behavior."]
// #[interoptopus::ffi_function]
// #[allow(unused_mut, unsafe_op_in_unsafe_fn, unused_unsafe)]
// #[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes, clippy::redundant_locals)]
// #[no_mangle]
// pub unsafe extern "C" fn service_async_destroy(context: &mut *mut ServiceAsync) -> FFIError {
//     if context.is_null() {
//         return <FFIError as ::interoptopus::patterns::result::FFIError>::NULL;
//     }
//     let result_result = ::std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
//         unsafe { drop(::std::boxed::Box::from_raw(*context)) };
//     }));
//     *context = ::std::ptr::null_mut();
//     match result_result {
//         Ok(_) => <FFIError as ::interoptopus::patterns::result::FFIError>::SUCCESS,
//         Err(e) => {
//             ::interoptopus::util::log_error(|| {
//                 format!(
//                     "Panic in ({}): {}",
//                     stringify!(service_async_destroy),
//                     ::interoptopus::patterns::result::get_panic_message(e.as_ref())
//                 )
//             });
//             <FFIError as ::interoptopus::patterns::result::FFIError>::PANIC
//         }
//     }
// }
// impl ::interoptopus::patterns::LibraryPatternInfo for ServiceAsync {
//     fn pattern_info() -> ::interoptopus::patterns::LibraryPattern {
//         use ::interoptopus::lang::rust::FunctionInfo;
//         let mut methods = Vec::new();
//         let mut ctors = Vec::new();
//         {
//             use service_async_return_after_ms as x;
//             methods.push(x::function_info());
//         }
//         {
//             use service_async_new as x;
//             ctors.push(x::function_info());
//         }
//         let dtor = {
//             use service_async_destroy as x;
//             x::function_info()
//         };
//         let service = ::interoptopus::patterns::service::Service::new(ctors, dtor, methods);
//         service.assert_valid();
//         ::interoptopus::patterns::LibraryPattern::Service(service)
//     }
// }
