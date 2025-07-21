use super::wire;
use interoptopus::{
    ffi_function, ffi_type,
    lang::{Wire, Wireable},
};
use std::collections::HashMap;

/// Main benchmark entry point for wire based interop.
#[ffi_function]
fn WireRustClient(mut wire_input: Wire<wire::Input>) -> Wire<wire::Outputs> {
    let input = wire_input.unwire().unwrap();
    let output = rust_client_impl(input);
    output.wire()
}

fn rust_client_impl(_input: Input) -> Outputs {
    // println!("rust_client_impl: {}, {}", _input.configuration.host, _input.configuration.response_size);

    let results = vec![Result { item_value: 42, item_id: "item1".to_string() }];
    let items = vec![Item { key: ItemKey::TOTAL, value: 100 }];
    Outputs { response: Response { results: results }, data: Data { items: Items { items }, errors: Error { error_messages: vec![] } } }
}

#[ffi_type(wired)]
pub struct Input {
    pub context: Context,
    pub value: Table,
    pub configuration: Configuration,
}

#[ffi_type(wired)]
pub struct Context {
    pub things: Vec<String>,
    pub headers: HashMap<String, String>,
}

#[ffi_type(wired)]
pub struct TableMetadata {
    pub row_count: i32,
    pub column_count: i32,
    pub guid: String,
    pub prefix: String,
}

#[ffi_type(wired)]
pub struct Table {
    pub metadata: TableMetadata,
    pub byte_array: Vec<u8>,
}

#[ffi_type(wired)]
pub struct Configuration {
    pub is_ok_response: bool,
    pub host: String,
    pub response_size: u64, // controls N in benchmarks
}

#[ffi_type(wired)]
pub struct Outputs {
    pub response: Response,
    pub data: Data,
}

#[ffi_type(wired)]
pub struct Result {
    pub item_id: String,
    pub item_value: i32,
}

#[ffi_type(wired)]
pub struct Response {
    pub results: Vec<Result>,
}

#[ffi_type(wired)]
pub struct Data {
    pub items: Items,
    pub errors: Error,
}

#[ffi_type(wired)]
pub struct Items {
    pub items: Vec<Item>,
}

#[ffi_type(wired)]
pub enum ItemKey {
    TOTAL = 0,
    FIRST = 1,
    SECOND = 2,
    THIRD = 3,
}

#[ffi_type(wired)]
pub struct Item {
    pub key: ItemKey,
    pub value: u64,
}

#[ffi_type(wired)]
pub struct Error {
    pub error_messages: Vec<String>,
}
