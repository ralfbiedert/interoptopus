//!
//! Interoptopus FFI-types based ipc
//!
use interoptopus::{ffi, ffi_function, ffi_type};

/// Main benchmark Rust entry point for FFI-based ipc.
#[ffi_function(debug)]
pub fn FfiRustClient(_input: Input) -> Outputs {
    // TODO: use input.response_size to generate outputs
    Outputs {
        response: Response { results: ffi::Vec::from_vec(vec![Result { item_value: 42, item_id: ffi::String::from("item1".to_string()) }]) },
        data: Data {
            items: Items { items: ffi::Vec::from_vec(vec![Item { key: ItemKey::TOTAL, value: 100 }]) },
            errors: Error { error_messages: ffi::Vec::from_vec(Vec::<ffi::String>::new()) },
        },
    }
}

#[ffi_type]
pub struct Input {
    context: Context,
    value: Table,
    configuration: Configuration,
}

#[ffi_type]
pub struct Outputs {
    response: Response,
    data: Data,
}

#[ffi_type]
pub struct Context {
    things: ffi::Vec<ffi::String>,
    // headers: HashMap<String, String>, // TODO: unsupported
}

#[ffi_type]
pub struct Table {
    metadata: TableMetadata,
    byte_array: ffi::Vec<u8>,
}

#[ffi_type]
pub struct Configuration {
    is_ok_response: bool,
    host: ffi::String,
    response_size: u64, // controls N in benchmarks
}

#[ffi_type]
pub struct Response {
    results: ffi::Vec<Result>,
}

#[ffi_type]
pub struct Data {
    items: Items,
    errors: Error,
}

#[ffi_type]
pub struct TableMetadata {
    row_count: i32,
    column_count: i32,
    guid: ffi::String,
    prefix: ffi::String,
}

#[ffi_type]
#[derive(Clone)]
pub struct Result {
    item_value: i32,
    item_id: ffi::String,
}

#[ffi_type]
pub struct Items {
    items: ffi::Vec<Item>,
}

#[ffi_type]
#[derive(Clone)]
pub enum ItemKey {
    TOTAL = 0,
    FIRST = 1,
    SECOND = 2,
    THIRD = 3,
}

#[ffi_type]
#[derive(Clone)]
pub struct Item {
    key: ItemKey,
    value: u64,
}

#[ffi_type]
pub struct Error {
    error_messages: ffi::Vec<ffi::String>,
}
