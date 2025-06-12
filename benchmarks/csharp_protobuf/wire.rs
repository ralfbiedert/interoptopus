// Wire type means types inside might not be a part of inventory - they are just
// used for serializing into a buffer.
#[ffi_function]
fn foo(i: Wire<Input>) -> Wire<Outputs> {
    let buf = i.wire();
    my_rust_function(buf);
    Wire::from(output)
}

trait Wired {
    fn ser(&self);
    fn de() -> Self;
    fn max_buffer_size(self) -> usize {
        // TODO: for some types we can't calculate ahead of time
        0usize //        4 + self.item_id.len() + 4;
    }
}

struct Wire<T> {
    buf: Vec<u8>, // ? who owns
    marker: PhantomData<T>,
}

// TODO: this should go through the C# generated shim that would unwire the result type
// private static unsafe extern void WireRustClient(byte[] structPointer, uint structLength, void** result, uint* resultLength);

use interoptopus::{ffi, ffi_type};
use std::collections::HashMap;

// use Wire<Input> in fn args
#[ffi_type(wired)]
struct Input {
    context: Context,
    value: Table,
    configuration: Configuration,
}

#[ffi_type(wired)]
struct Context {
    things: Vec<String>,
    headers: HashMap<String, String>,
}

#[ffi_type(wired)]
struct TableMetadata {
    row_count: i32,
    column_count: i32,
    guid: String,
    prefix: String,
}

#[ffi_type(wired)]
struct Table {
    metadata: TableMetadata,
    byteArray: Vec<u8>,
}

#[ffi_type(wired)]
struct Configuration {
    is_ok_response: bool,
    host: String,
    response_size: u64, // controls N in benchmarks
}

// use Wire<Outputs> in fn args
#[ffi_type(wired)]
struct Outputs {
    response: Response,
    data: Data,
}

// #[ffi_type]
// struct Result {
//     item_id: ffi::String, // <- in inventory
//     item_value: i32,
// }

#[ffi_type(wired)] // <-- it's a Wired type
struct Result {
    item_id: String, // <- not in inventory anymore, just a type to write to a buf
    item_value: i32,
}

// class Result {
//     public string item_id; // <- not in inventory anymore, just a type to write to a buf
//     public int item_value;
// }

// class WireResult {
//     public static Result Deserialize() {
//         // Read String first.
//         // Read i32 next.
//     }
//     public static WireResult Serialize() {
//     }
// }

// public static Bar() {
//     WireResult x = Foo();
//     var xx = x.Deserialize();

//     Foo2(xx.Serialize())
// }

#[ffi_type(wired)]
struct Response {
    results: Vec<Result>,
}

#[ffi_type(wired)]
struct Data {
    items: Items,
    errors: Error,
}

#[ffi_type(wired)]
struct Items {
    items: Vec<Item>,
}

#[ffi_type(wired)]
enum ItemKey {
    TOTAL = 0,
    FIRST = 1,
    SECOND = 2,
    THIRD = 3,
}

#[ffi_type(wired)]
struct Item {
    key: ItemKey,
    value: u64,
}

#[ffi_type(wired)]
struct Error {
    error_messages: Vec<String>,
}
