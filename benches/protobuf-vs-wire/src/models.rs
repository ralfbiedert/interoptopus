use interoptopus::{ffi, ffi_type};
use std::collections::HashMap;

// use Wire<Input> in fn args
#[ffi_type]
struct Input {
    context: Context,
    value: Table,
    configuration: Configuration,
}

#[ffi_type]
struct Context {
    things: Vec<String>,
    headers: HashMap<String, String>,
}

#[ffi_type]
struct TableMetadata {
    rowCount: i32,
    columnCount: i32,
    guid: String,
    prefix: String,
}

#[ffi_type]
struct Table {
    metadata: TableMetadata,
    byteArray: Vec<u8>,
}

#[ffi_type]
struct Configuration {
    is_local_test: bool,
    host: String,
}

// Wire<Outputs>
#[ffi_type]
struct Outputs {
    response: Response,
    data: Data,
}

#[ffi_type(wired)] // <-- it's a Wired type
struct Result {
    item_id: String, // <- not in inventory anymore, just a type to write to a buf
    item_value: i32,
}

#[ffi_type]
struct Some {
    item_id: String, // <- in inventory
    item_value: i32,
}

#[ffi_type]
struct Response {
    results: Vec<Result>,
}

#[ffi_type]
struct Data {
    items: Items,
    errors: Error,
}

#[ffi_type]
struct Items {
    items: Vec<Item>,
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
    error_messages: Vec<String>,
}
