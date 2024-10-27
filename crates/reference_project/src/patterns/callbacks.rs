use crate::patterns::result::FFIError;
use interoptopus::{callback_immediate, callback_retained, ffi_function, ffi_type};
use std::ffi::c_void;

callback_immediate!(Callback(value: u32) -> u32);
callback_immediate!(CallbackNamespaced(value: u32) -> u32, namespace = "common");
callback_immediate!(CallbackVoid(ptr: *const c_void));
callback_immediate!(CallbackContextual(context: *const c_void, value: u32));
callback_immediate!(CallbackSum1());
callback_immediate!(CallbackSum2(x: i32, y: i32) -> i32);
callback_immediate!(CallbackError(x: i32, y: i32) -> FFIError);

callback_retained!(CallbackRetained(value: u32) -> u32);
callback_retained!(CallbackErrorRetained(x: i32, y: i32) -> FFIError);
callback_retained!(CallbackNamespacedRetained(value: u32) -> u32, namespace = "common");

#[ffi_type]
pub struct DelegateCallback<C> {
    pub callback: C,
    pub context: *const c_void,
}

#[ffi_function]
pub fn pattern_callback_1(callback: Callback, x: u32) -> u32 {
    callback.call(x)
}

#[ffi_function]
pub fn pattern_callback_2(callback: CallbackVoid) -> CallbackVoid {
    callback
}

#[ffi_function]
pub fn pattern_callback_3(callback: DelegateCallback<CallbackContextual>, x: u32) {
    callback.callback.call(callback.context, x);
}

#[ffi_function]
pub fn pattern_callback_4(callback: CallbackNamespaced, x: u32) -> u32 {
    callback.call(x)
}

#[ffi_function]
pub fn pattern_callback_5() -> CallbackSum1<'static> {
    CallbackSum1::new(exposed_sum1)
}

#[ffi_function]
pub fn pattern_callback_6() -> CallbackSum2<'static> {
    CallbackSum2::new(exposed_sum2)
}

#[ffi_function]
pub fn pattern_callback_7(c1: CallbackError, c2: CallbackError, x: i32, i: i32, o: &mut i32) -> FFIError {
    *o = i - 1;

    // Call both callbacks. In C#, if the callback throws an exception, we might not re-enter
    // and the rest of this function won't run (incl. not running `drop()` and doing other
    // cleanup.
    //
    // Callbacks that return an `FFIError` can (if enabled in the C# backend) avoid that issue
    // by doing some exception handling ping-pong; see the interoptopus_backend_csharp
    // config setting `config.work_around_exception_in_callback_no_reentry`.
    //
    c1.call(x, x); // In a real world you'd also want to check the result here.
    c2.call(x, x);

    *o = i + 1;

    FFIError::Ok
}

pub extern "C" fn exposed_sum1() {}

pub extern "C" fn exposed_sum2(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    use super::{Callback, CallbackNamespaced};
    use interoptopus::lang::rust::CTypeInfo;

    #[test]
    fn namespaces_assigned_correctly() {
        let ti1 = Callback::type_info();
        let ti2 = CallbackNamespaced::type_info();

        assert_eq!(ti1.namespace(), Some(""));
        assert_eq!(ti2.namespace(), Some("common"));
    }
}
