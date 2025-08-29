use crate::patterns::result::Error;
use crate::types::string::UseString;
use interoptopus::lang::NAMESPACE_COMMON;
use interoptopus::{callback, ffi, ffi_function, ffi_type};
use std::ffi::c_void;
use std::ptr::null;

callback!(MyCallback(value: u32) -> u32);
callback!(MyCallbackNamespaced(value: u32) -> u32, namespace = NAMESPACE_COMMON);
callback!(MyCallbackVoid(ptr: *const c_void));
callback!(MyCallbackContextual(context: *const c_void, value: u32));
callback!(SumDelegate1());
callback!(SumDelegate2(x: i32, y: i32) -> i32);
callback!(SumDelegateReturn(x: i32, y: i32) -> ffi::Result<(), Error>);
callback!(SumDelegateReturn2(x: i32, y: i32));
callback!(StringCallback(s: ffi::String));
callback!(NestedStringCallback(s: UseString));

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

// #[ffi_function]
// pub fn pattern_callback_3(callback: DelegateCallback<MyCallbackContextual>, x: u32) {
//     callback.callback.call(callback.context, x);
// }

#[ffi_function]
pub fn pattern_callback_4(callback: MyCallbackNamespaced, x: u32) -> u32 {
    callback.call(x)
}

#[ffi_function]
pub fn pattern_callback_5() -> SumDelegate1 {
    (exposed_sum1 as extern "C" fn(*const c_void)).into() // This is an ugly Rust limitation right now, compare #108
}

#[ffi_function]
pub fn pattern_callback_6() -> SumDelegate2 {
    SumDelegate2(Some(exposed_sum2), null()) // Similarly, compare #108
}

#[ffi_function]
pub fn pattern_callback_7(c1: SumDelegateReturn, c2: SumDelegateReturn2, x: i32, i: i32, o: &mut i32) -> ffi::Result<(), Error> {
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

    ffi::Ok(())
}

#[ffi_function]
pub fn pattern_callback_8(cb: StringCallback, cb2: NestedStringCallback, s: ffi::String) {
    cb.call(s.clone());
    cb2.call(UseString { s1: s.clone(), s2: s.clone() });
}

pub extern "C" fn exposed_sum1(x: *const c_void) {
    println!("0x{x:?}");
    eprintln!("0x{x:?}");
}

pub extern "C" fn exposed_sum2(x: i32, y: i32, _: *const c_void) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    use super::{MyCallback, MyCallbackNamespaced};
    use interoptopus::lang::{NAMESPACE_COMMON, TypeInfo};

    #[test]
    fn namespaces_assigned_correctly() {
        let ti1 = MyCallback::type_info();
        let ti2 = MyCallbackNamespaced::type_info();

        assert_eq!(ti1.namespace(), Some(""));
        assert_eq!(ti2.namespace(), Some(NAMESPACE_COMMON));
    }
}
