use interoptopus::{
    ffi_function, ffi_type,
    lang::{Wire, Wireable},
};
use std::collections::HashMap;

/// Main benchmark entry point for wire based interop.
#[ffi_function(namespace = "wire", debug)]
fn WireRustClient(mut wire_input: Wire<WInput>) -> Wire<WOutputs> {
    let input = wire_input.unwire().unwrap();
    let output = rust_client_impl(input);
    output.wire()
}

fn rust_client_impl(_input: WInput) -> WOutputs {
    // println!("rust_client_impl: {}, {}", _input.configuration.host, _input.configuration.response_size);

    let results = vec![WResult { item_value: 42, item_id: "item1".to_string() }];
    let items = vec![WItem { key: WItemKey::TOTAL, value: 100 }];
    WOutputs { response: WResponse { results: results }, data: WData { items: WItems { items }, errors: None } }
}

#[ffi_type(wired, namespace = "wire", debug)]
pub struct WInput {
    pub context: WContext,
    pub value: WTable,
    pub configuration: WConfiguration,
}

#[ffi_type(wired, namespace = "wire")]
pub struct WContext {
    pub things: Vec<String>,
    pub headers: HashMap<String, String>,
}

#[ffi_type(wired, namespace = "wire")]
pub struct WTableMetadata {
    pub row_count: i32,
    pub column_count: i32,
    pub guid: String,
    pub prefix: String,
}

#[ffi_type(wired, namespace = "wire")]
pub struct WTable {
    pub metadata: WTableMetadata,
    pub byte_array: Vec<u8>,
}

#[ffi_type(wired, namespace = "wire")]
pub struct WConfiguration {
    pub is_ok_response: bool,
    pub host: String,
    pub response_size: u64, // controls N in benchmarks
}

#[ffi_type(wired, namespace = "wire", debug)]
pub struct WOutputs {
    pub response: WResponse,
    pub data: WData,
}

#[ffi_type(wired, namespace = "wire")]
pub struct WResult {
    pub item_id: String,
    pub item_value: i32,
}

#[ffi_type(wired, namespace = "wire")]
pub struct WResponse {
    pub results: Vec<WResult>,
}

#[ffi_type(wired, namespace = "wire")]
pub struct WData {
    pub items: WItems,
    pub errors: Option<WError>,
}

#[ffi_type(wired, namespace = "wire")]
pub struct WItems {
    pub items: Vec<WItem>,
}

#[ffi_type(wired, namespace = "wire")]
pub enum WItemKey {
    TOTAL = 0,
    FIRST = 1,
    SECOND = 2,
    THIRD = 3,
}

#[ffi_type(wired, namespace = "wire")]
pub struct WItem {
    pub key: WItemKey,
    pub value: u64,
}

#[ffi_type(wired, namespace = "wire")]
pub struct WError {
    pub error_messages: Vec<String>,
}
