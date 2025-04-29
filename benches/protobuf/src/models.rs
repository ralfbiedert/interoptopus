struct Input {
    context: Context,
    value: Table,
    configuration: Configuration,
}

struct Context {
    things: Vec<String>,
    headers: HashMap<String, String>,
}

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

struct Outputs {
    response: Response,
    data: Data,
}

struct Result {
    item_id: String,
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
