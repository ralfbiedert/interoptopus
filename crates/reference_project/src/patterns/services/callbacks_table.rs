use crate::patterns::callbacks::{CallbackErrorRetained, CallbackNamespacedRetained, CallbackRetained};
use crate::patterns::result::{Error, FFIError};
use interoptopus::{ffi_service, ffi_service_ctor, ffi_type};

#[ffi_type]
#[derive(Copy, Clone)]
pub struct CallbackTable {
    pub error: CallbackErrorRetained,
    pub callback: CallbackRetained,
    pub namespaced: CallbackNamespacedRetained,
}

/// Some struct we want to expose as a class.
#[ffi_type(opaque)]
pub struct ServiceCallbacksTable {
    delegate_table: Option<CallbackTable>,
}

// Regular implementation of methods.
#[ffi_service(error = "FFIError")]
impl ServiceCallbacksTable {
    #[ffi_service_ctor]
    pub fn new() -> Result<Self, Error> {
        Ok(Self { delegate_table: None })
    }

    #[ffi_service_ctor]
    pub fn new_with_table(table: CallbackTable) -> Result<Self, Error> {
        Ok(Self { delegate_table: Some(table) })
    }

    pub fn set_callback_table(&mut self, table: &CallbackTable) {
        self.delegate_table = Some(*table);
    }

    pub fn invoke_callbacks(&self) -> Result<(), Error> {
        let Some(table) = &self.delegate_table else {
            return Ok(());
        };

        table.callback.call(32);
        table.namespaced.call(32);
        table.error.call(32, 32);

        Ok(())
    }
}
