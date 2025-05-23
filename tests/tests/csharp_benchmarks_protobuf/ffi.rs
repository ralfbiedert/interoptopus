//!
//! Interoptopus FFI-types based ipc
//!
use interoptopus::{ffi, ffi_function, ffi_type};

#[ffi_function]
pub fn FfiRustClient(_input: Input) -> Outputs {
    // TODO: use input to generate outputs
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
struct Context {
    things: ffi::Vec<ffi::String>,
    // headers: HashMap<String, String>, // TODO: unsupported
}

#[ffi_type]
struct Table {
    metadata: TableMetadata,
    byte_array: ffi::Vec<u8>,
}

#[ffi_type]
struct Configuration {
    is_local_test: bool,
    host: ffi::String,
}

#[ffi_type]
struct Response {
    results: ffi::Vec<Result>,
}

#[ffi_type]
struct Data {
    items: Items,
    errors: Error,
}

#[ffi_type]
struct TableMetadata {
    row_count: i32,
    column_count: i32,
    guid: ffi::String,
    prefix: ffi::String,
}

#[ffi_type]
struct Result {
    item_value: i32,
    item_id: ffi::String,
}

#[ffi_type]
struct Items {
    items: ffi::Vec<Item>,
}

#[ffi_type]
enum ItemKey {
    TOTAL = 0,
    FIRST = 1,
    SECOND = 2,
    THIRD = 3,
}

#[ffi_type]
struct Item {
    key: ItemKey,
    value: u64,
}

#[ffi_type]
struct Error {
    error_messages: ffi::Vec<ffi::String>,
}
