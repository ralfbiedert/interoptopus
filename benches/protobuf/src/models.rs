// Wire<Input>
struct Input {
    context: Context,
    value: Table,
    configuration: Configuration,
}

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

struct Table {
    metadata: TableMetadata,
    byteArray: Vec<u8>,
}

struct Configuration {
    is_local_test: bool,
    host: String,
}

// Wire<Outputs>
#[ffi_type] for all
struct Outputs {
    response: Response,
    data: Data,
}

// Wire type means types inside might not be a part of inventory - they are just
// used for serializing into a buffer.
#[ffi_function]
fn foo(i: Wire<Input>) -> Wire<Outputs> {
    let buf = i.wire();
    my_rust_function(buf);
    Wire::from(output)
}

trait Wired {
    fn ser();
    fn de();
    fn max_buffer_size(self) -> usize { 4+self.item_id.len()+4; }
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

struct Response {
    results: Vec<Result>,
}

struct Data {
    items: Items,
    errors: Error,
}

struct Items {
    items: Vec<Item>,
}

enum ItemKey {
    TOTAL = 0,
    FIRST = 1,
    SECOND = 2,
    THIRD = 3,
}

struct Item {
    key: ItemKey,
    value: u64,
}

struct Error {
    error_messages: Vec<String>,
}

#[ffi_type(wired)]
struct Weird {
    items: Wire<Vec<Item>>,
}

struct Wire<T> {
    buf: Vec<u8>, // ? who owns
    marker: PhantomData<T>
}
