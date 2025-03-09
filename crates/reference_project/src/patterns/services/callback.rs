use crate::patterns::callback::{
    MyCallback, MyCallbackContextual, MyCallbackNamespaced, MyCallbackVoid, SumDelegate1, SumDelegate2, SumDelegateReturn, SumDelegateReturn2,
};
use crate::patterns::result::FFIError;
use interoptopus::patterns::result::FFIResult;
use interoptopus::patterns::slice::FFISlice;
use interoptopus::{ffi_service, ffi_type};

#[ffi_type]
#[derive(Copy, Clone)]
pub struct CallbackTable {
    pub my_callback: MyCallback,
    pub my_callback_namespaced: MyCallbackNamespaced,
    pub my_callback_void: MyCallbackVoid,
    pub my_callback_contextual: MyCallbackContextual,
    pub sum_delegate_1: SumDelegate1,
    pub sum_delegate_2: SumDelegate2,
    pub sum_delegate_return: SumDelegateReturn,
    pub sum_delegate_return_2: SumDelegateReturn2,
}

/// Some struct we want to expose as a class.
#[ffi_type(opaque)]
pub struct ServiceCallbacks {
    delegate_table: Option<CallbackTable>,
}

// Regular implementation of methods.
#[ffi_service]
impl ServiceCallbacks {
    pub fn new() -> FFIResult<Self, FFIError> {
        FFIResult::ok(Self { delegate_table: None })
    }

    pub fn callback_simple(&mut self, callback: MyCallback) -> FFIResult<(), FFIError> {
        callback.call(0);
        FFIResult::ok(())
    }

    pub fn callback_ffi_return(&mut self, callback: SumDelegateReturn) -> FFIResult<(), FFIError> {
        callback.call(0, 0);
        FFIResult::ok(())
    }

    pub fn callback_with_slice(&mut self, callback: SumDelegateReturn, input: FFISlice<i32>) -> FFIResult<(), FFIError> {
        callback.call(input.as_slice()[0], input.as_slice()[1]);
        FFIResult::ok(())
    }

    pub fn set_delegate_table(&mut self, table: CallbackTable) {
        self.delegate_table = Some(table);
    }

    pub fn invoke_delegates(&self) -> FFIResult<(), FFIError> {
        let Some(table) = &self.delegate_table else {
            return FFIResult::ok(());
        };

        table.my_callback.call(123);
        table.sum_delegate_1.call();
        table.sum_delegate_2.call(123, 123);
        table.sum_delegate_return.call(123, 123);

        FFIResult::ok(())
    }
}
