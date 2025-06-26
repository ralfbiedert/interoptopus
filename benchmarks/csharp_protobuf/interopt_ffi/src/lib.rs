//!
//! Interoptopus FFI-types based ipc
//!
use interoptopus::{
    builtins_string, builtins_vec, ffi, ffi_function, ffi_type, function,
    inventory::{Inventory, InventoryBuilder},
};
// use interoptopus::patterns::slice::FFISlice;

pub fn ffi_inventory() -> Inventory {
    InventoryBuilder::new()
        .register(builtins_string!())
        .register(builtins_vec!(u8))
        .register(builtins_vec!(ffi::String))
        .register(function!(FfiRustClient))
        .register(builtins_vec!(Item))
        .register(builtins_vec!(Result))
        .validate()
        .build()
}

/// Main benchmark Rust entry point for FFI-based ipc.
#[ffi_function]
pub fn FfiRustClient(_input: Input) -> Outputs {
    /*
    println!("PRINTLN DEBUG IS THA BEST");
    println!("NUMBERS VALIDITY CHECK:");
    println!("response_size = {}", _input.configuration.response_size);
    println!("is_ok_response = {}", _input.configuration.is_ok_response);
    println!("row_count = {}", _input.value.metadata.row_count);
    println!("column_count = {}", _input.value.metadata.column_count);

    println!("HOST RECEIVED: {:?}", _input.configuration.host);*/
    // TODO: use input.response_size to generate outputs
    let results = vec![Result { item_value: 42, item_id: ffi::String::from("item1".to_string()) }];
    let items = vec![Item { key: ItemKey::TOTAL, value: 100 }];
    Outputs {
        response: Response { results: ffi::Vec::from_vec(results) },
        data: Data { items: Items { items: ffi::Vec::from_vec(items) }, errors: Error { error_messages: ffi::Vec::from_vec(Vec::<ffi::String>::new()) } },
    }
}

#[ffi_type]
pub struct Input<'l> {
    pub context: Context<'l>,
    pub value: Table<'l>,
    pub configuration: Configuration,
}

#[ffi_type]
pub struct Outputs {
    pub response: Response,
    pub data: Data,
}

#[ffi_type]
pub struct Context<'l> {
    pub things: ffi::Slice<'l, ffi::String>,
    // headers: HashMap<String, String>, // TODO: unsupported
}

#[ffi_type]
pub struct Table<'l> {
    pub metadata: TableMetadata,
    pub byte_array: ffi::Slice<'l, u8>,
}

#[ffi_type]
pub struct Configuration {
    pub is_ok_response: bool,
    pub host: ffi::String,
    pub response_size: u64, // controls N in benchmarks
}

#[ffi_type]
pub struct Response {
    pub results: ffi::Vec<Result>,
}

#[ffi_type]
pub struct Data {
    pub items: Items,
    pub errors: Error,
}

#[ffi_type]
pub struct TableMetadata {
    pub row_count: i32,
    pub column_count: i32,
    pub guid: ffi::String,
    pub prefix: ffi::String,
}

#[ffi_type]
#[derive(Clone)]
pub struct Result {
    pub item_value: i32,
    pub item_id: ffi::String,
}

#[ffi_type]
pub struct Items {
    pub items: ffi::Vec<Item>,
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
    pub key: ItemKey,
    pub value: u64,
}

#[ffi_type]
pub struct Error {
    pub error_messages: ffi::Vec<ffi::String>,
}
