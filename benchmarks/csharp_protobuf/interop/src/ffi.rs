use interoptopus::{ffi, ffi_function, ffi_type};

/// Main benchmark Rust entry point for FFI-based interop.
#[ffi_function(namespace = "ffi")]
pub fn FfiRustClient(_input: FInput) -> FOutputs {
    /*    println!("PRINTLN DEBUG IS THA BEST");
    println!("NUMBERS VALIDITY CHECK:");
    println!("response_size = {}", _input.configuration.response_size);
    println!("is_ok_response = {}", _input.configuration.is_ok_response);
    println!("row_count = {}", _input.value.metadata.row_count);
    println!("column_count = {}", _input.value.metadata.column_count);

    println!("HOST RECEIVED: {:?}", _input.configuration.host);*/

    // TODO: use input.response_size to generate outputs
    let results = vec![FResult { item_value: 42, item_id: ffi::String::from("item1".to_string()) }];
    let items = vec![FItem { key: FItemKey::TOTAL, value: 100 }];
    FOutputs {
        response: FResponse { results: ffi::Vec::from_vec(results) },
        data: FData { items: FItems { items: ffi::Vec::from_vec(items) }, errors: FError { error_messages: ffi::Vec::from_vec(Vec::<ffi::String>::new()) } },
    }
}

#[ffi_type(namespace = "ffi")]
pub struct FInput<'l> {
    pub context: FContext<'l>,
    pub value: FTable<'l>,
    pub configuration: FConfiguration,
}

#[ffi_type(namespace = "ffi")]
pub struct FOutputs {
    pub response: FResponse,
    pub data: FData,
}

#[ffi_type(namespace = "ffi")]
pub struct FContext<'l> {
    pub things: ffi::Slice<'l, ffi::String>,
    // headers: HashMap<String, String>, // TODO: unsupported
}

#[ffi_type(namespace = "ffi")]
pub struct FTable<'l> {
    pub metadata: FTableMetadata,
    pub byte_array: ffi::Slice<'l, u8>,
}

#[ffi_type(namespace = "ffi")]
pub struct FConfiguration {
    pub is_ok_response: bool,
    pub host: ffi::String,
    pub response_size: u64, // controls N in benchmarks
}

#[ffi_type(namespace = "ffi")]
pub struct FResponse {
    pub results: ffi::Vec<FResult>,
}

#[ffi_type(namespace = "ffi")]
pub struct FData {
    pub items: FItems,
    pub errors: FError,
}

#[ffi_type(namespace = "ffi")]
pub struct FTableMetadata {
    pub row_count: i32,
    pub column_count: i32,
    pub guid: ffi::String,
    pub prefix: ffi::String,
}

#[ffi_type(namespace = "ffi")]
#[derive(Clone)]
pub struct FResult {
    pub item_value: i32,
    pub item_id: ffi::String,
}

#[ffi_type(namespace = "ffi")]
pub struct FItems {
    pub items: ffi::Vec<FItem>,
}

#[ffi_type(namespace = "ffi")]
#[derive(Clone)]
pub enum FItemKey {
    TOTAL = 0,
    FIRST = 1,
    SECOND = 2,
    THIRD = 3,
}

#[ffi_type(namespace = "ffi")]
#[derive(Clone)]
pub struct FItem {
    pub key: FItemKey,
    pub value: u64,
}

#[ffi_type(namespace = "ffi")]
pub struct FError {
    pub error_messages: ffi::Vec<ffi::String>,
}
