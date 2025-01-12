use crate::patterns::callbacks::{
    MyCallback, MyCallbackContextual, MyCallbackNamespaced, MyCallbackVoid, SumDelegate1, SumDelegate2, SumDelegateReturn, SumDelegateReturn2,
};
use crate::patterns::result::{Error, FFIError};
use interoptopus::patterns::slice::FFISlice;
use interoptopus::{ffi_service, ffi_service_ctor, ffi_type};

#[ffi_type]
#[derive(Copy, Clone)]
pub struct DelegateTable {
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
    delegate_table: Option<DelegateTable>,
}

// Regular implementation of methods.
#[ffi_service(error = "FFIError")]
impl ServiceCallbacks {
    #[ffi_service_ctor]
    pub fn new() -> Result<Self, Error> {
        Ok(Self { delegate_table: None })
    }

    pub fn callback_simple(&mut self, callback: MyCallback) -> Result<(), Error> {
        callback.call(0);
        Ok(())
    }

    pub fn callback_ffi_return(&mut self, callback: SumDelegateReturn) -> Result<(), Error> {
        callback.call(0, 0);
        Ok(())
    }

    pub fn callback_with_slice(&mut self, callback: SumDelegateReturn, input: FFISlice<i32>) -> Result<(), Error> {
        callback.call(input.as_slice()[0], input.as_slice()[1]);
        Ok(())
    }

    // UNSUPPORTED FOR NOW - Unclear how to handle in C# with LibraryImport
    // pub fn set_delegate_table(&mut self, table: &DelegateTable) {
    //     self.delegate_table = Some(*table);
    // }

    pub fn invoke_delegates(&self) -> Result<(), Error> {
        let Some(table) = &self.delegate_table else {
            return Ok(());
        };

        table.my_callback.call(123);
        table.sum_delegate_1.call();
        table.sum_delegate_2.call(123, 123);
        table.sum_delegate_return.call(123, 123);

        Ok(())
    }
}
