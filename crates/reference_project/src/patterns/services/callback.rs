use crate::patterns::callback::{
    MyCallback, MyCallbackContextual, MyCallbackNamespaced, MyCallbackVoid, SumDelegate1, SumDelegate2, SumDelegateReturn, SumDelegateReturn2,
};
use crate::patterns::result::FFIError;
use interoptopus::{ffi, ffi_service, ffi_type};

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
    pub fn new() -> ffi::Result<Self, FFIError> {
        ffi::Ok(Self { delegate_table: None })
    }

    pub fn callback_simple(&mut self, callback: MyCallback) -> ffi::Result<(), FFIError> {
        callback.call(0);
        ffi::Ok(())
    }

    pub fn callback_ffi_return(&mut self, callback: SumDelegateReturn) -> ffi::Result<(), FFIError> {
        callback.call(0, 0);
        ffi::Ok(())
    }

    pub fn callback_with_slice(&mut self, callback: SumDelegateReturn, input: ffi::Slice<i32>) -> ffi::Result<(), FFIError> {
        callback.call(input.as_slice()[0], input.as_slice()[1]);
        ffi::Ok(())
    }

    pub fn set_delegate_table(&mut self, table: CallbackTable) {
        self.delegate_table = Some(table);
    }

    pub fn invoke_delegates(&self) -> ffi::Result<(), FFIError> {
        let Some(table) = &self.delegate_table else {
            return ffi::Ok(());
        };

        table.my_callback.call(123);
        table.sum_delegate_1.call();
        table.sum_delegate_2.call(123, 123);
        table.sum_delegate_return.call(123, 123);

        ffi::Ok(())
    }
}
