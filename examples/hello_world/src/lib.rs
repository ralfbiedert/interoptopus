use interoptopus::{callback, ffi, ffi_function, ffi_type, wire::Wire};
use std::collections::HashMap;

callback!(CsharpCallback(result: interoptopus::ffi::Slice<u8>));

#[ffi_type(wired)]
pub struct Pair {
    pub key: u32,
    pub value: Option<String>,
}

impl std::fmt::Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.key, self.value.as_deref().unwrap_or("??"))
    }
}

#[ffi_type(wired)]
pub struct Fields {
    pub name: String,
    pub attrs: HashMap<String, Pair>, // Demo: replace String with Pair
}

#[ffi_function]
pub fn concat(mut input: Wire<Fields>, callback: CsharpCallback) -> ffi::String {
    let fields = input.unwire().expect("All shall be good whoohoo");

    let output = format!("{}: {}", fields.name, fields.attrs.iter().fold(String::new(), |acc, (k, v)| format!("{acc} {k}={v}")));

    callback.call(interoptopus::ffi::Slice::from_slice(output.as_bytes()));

    ffi::String::from_string(output)
}

// We just trick a unit test into producing our bindings, here for C#
#[test]
#[rustfmt::skip]
fn generate_bindings() {
    use interoptopus::function;
    use interoptopus::inventory::Inventory;
    use interoptopus_backend_csharp::Interop;

    // In a real project this should be a freestanding `my_inventory()` function inside
    // your FFI or build crate.
    let inventory = Inventory::builder()
        .register(function!(concat))
        .validate()
        .build();

    Interop::builder()
        .inventory(inventory)
        .build().unwrap()
        .write_file("bindings/Interop.cs").unwrap()
}
