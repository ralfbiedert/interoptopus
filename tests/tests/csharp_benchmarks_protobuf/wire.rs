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
    fn max_buffer_size(self) -> usize { // TODO: for some types we can't calculate ahead of time
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
    response_size_factor: u64, // controls N in benchmarks
}

// Wire<Outputs>
#[ffi_type]
struct Outputs {
    response: Response,
    data: Data,
}

#[ffi_type(wired)] // <-- it's a Wired type
struct Result {
    item_value: i32,
    item_id: String, // <- not in inventory anymore, just a type to write to a buf
}

class Result {
    public int item_value;
    public string item_id; // <- not in inventory anymore, just a type to write to a buf
}

class WireResult {
    public static Result Deserialize() {
        // Read i32 first.


        // Read String next.

    }

    public static WireResult Serialize() {

    }
}

public static Bar() {
    WireResult x = Foo();
    var xx = x.Deserialize();

    Foo2(xx.Serialize())
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
