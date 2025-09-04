use interoptopus::{callback, ffi, ffi_function, ffi_type, function, inventory::Inventory};

pub fn ffi_inventory() -> Inventory {
    Inventory::builder().register(function!(init_logger)).validate().build()
}

#[ffi_type]
pub struct KVPair<'a> {
    key: ffi::Slice<'a, u8>,
    value: ffi::Slice<'a, u8>,
}

callback!(LoggerCallback(fields: ffi::Vec<KVPair<'_>>));

#[ffi_function]
pub extern "C" fn init_logger(_callback: LoggerCallback) {}
