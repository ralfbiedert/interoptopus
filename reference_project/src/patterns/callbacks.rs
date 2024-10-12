use crate::patterns::result::{Error, FFIError};
use interoptopus::patterns::result::FFIDelegateError;
use interoptopus::{callback, ffi_function, ffi_type};
use std::ffi::c_void;

callback!(MyCallback(value: u32) -> u32);
callback!(MyCallbackNamespaced(value: u32) -> u32, namespace = "common");
callback!(MyCallbackVoid(ptr: *const c_void));
callback!(MyCallbackContextual(context: *const c_void, value: u32));
callback!(SumDelegate1());
callback!(SumDelegate2(x: i32, y: i32) -> i32);
callback!(SumDelegateReturn(x: i32, y: i32) -> FFIError);

#[ffi_type]
pub struct DelegateCallback<C> {
    pub callback: C,
    pub context: *const c_void,
}

#[ffi_function]
pub fn pattern_callback_1(callback: MyCallback, x: u32) -> u32 {
    callback.call(x)
}

#[ffi_function]
pub fn pattern_callback_2(callback: MyCallbackVoid) -> MyCallbackVoid {
    callback
}

#[ffi_function]
pub fn pattern_callback_3(callback: DelegateCallback<MyCallbackContextual>, x: u32) {
    callback.callback.call(callback.context, x);
}

#[ffi_function]
pub fn pattern_callback_4(callback: MyCallbackNamespaced, x: u32) -> u32 {
    callback.call(x)
}

#[ffi_function]
pub fn pattern_callback_5() -> SumDelegate1 {
    (exposed_sum1 as extern "C" fn()).into() // This is an ugly Rust limitation right now, compare #108
}

#[ffi_function]
pub fn pattern_callback_6() -> SumDelegate2 {
    SumDelegate2(Some(exposed_sum2)) // Similarly, compare #108
}

#[ffi_function]
pub fn pattern_callback_7(callback: SumDelegateReturn, x: i32) -> FFIError {
    // So the basic requirement here is that during that call
    // the trampoline would catch an exception, then
    // signal-return that an exception happened, then stop resuming
    // what it does and return early itself, and then the function
    // that actually invoked this one would pick that callback up
    // and re-throw it.
    match callback.call(x, x).ok() {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }
}

pub extern "C" fn exposed_sum1() {}

pub extern "C" fn exposed_sum2(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    use super::{MyCallback, MyCallbackNamespaced};
    use interoptopus::lang::rust::CTypeInfo;

    #[test]
    fn namespaces_assigned_correctly() {
        let ti1 = MyCallback::type_info();
        let ti2 = MyCallbackNamespaced::type_info();

        assert_eq!(ti1.namespace(), Some(""));
        assert_eq!(ti2.namespace(), Some("common"));
    }
}
