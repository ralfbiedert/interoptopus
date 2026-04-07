use crate::patterns::callback::{
    MyCallback, MyCallbackContextual, MyCallbackNamespaced, MyCallbackVoid, SumDelegate1, SumDelegate2, SumDelegateReturn, SumDelegateReturn2,
};
use crate::patterns::result::Error;
use interoptopus::{ffi, ffi::*};

#[ffi]
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
#[ffi(service)]
pub struct ServiceCallbacks {
    delegate_table: Option<CallbackTable>,
    stored_callback: Option<MyCallback>,
}

// Regular implementation of methods.
#[ffi]
impl ServiceCallbacks {
    pub fn create() -> ffi::Result<Self, Error> {
        ffi::Ok(Self { delegate_table: None, stored_callback: None })
    }

    /// Constructor that takes a callback and stores it for later invocation.
    /// This tests that the generated C# service class keeps the managed
    /// delegate alive (prevents GC) for the lifetime of the service.
    pub fn create_with_callback(callback: MyCallback) -> ffi::Result<Self, Error> {
        ffi::Ok(Self { delegate_table: None, stored_callback: Some(callback) })
    }

    /// Invoke the callback stored at construction time.
    pub fn invoke_stored_callback(&self, x: u32) -> u32 {
        match &self.stored_callback {
            Some(cb) => cb.call(x),
            None => 0,
        }
    }

    pub fn callback_simple(&mut self, callback: MyCallback) -> ffi::Result<(), Error> {
        callback.call(0);
        ffi::Ok(())
    }

    pub fn callback_ffi_return(&mut self, callback: SumDelegateReturn) -> ffi::Result<(), Error> {
        _ = callback.call(0, 0);
        ffi::Ok(())
    }

    pub fn callback_with_slice(&mut self, callback: SumDelegateReturn, input: ffi::Slice<i32>) -> ffi::Result<(), Error> {
        _ = callback.call(input.as_slice()[0], input.as_slice()[1]);
        ffi::Ok(())
    }

    pub fn set_delegate_table(&mut self, table: CallbackTable) {
        self.delegate_table = Some(table);
    }

    pub fn invoke_delegates(&self) -> ffi::Result<(), Error> {
        let Some(table) = &self.delegate_table else {
            return ffi::Ok(());
        };

        table.my_callback.call(123);
        table.sum_delegate_1.call();
        table.sum_delegate_2.call(123, 123);
        _ = table.sum_delegate_return.call(123, 123);

        ffi::Ok(())
    }
}
