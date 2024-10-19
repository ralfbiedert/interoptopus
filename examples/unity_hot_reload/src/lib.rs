use interoptopus::{ffi_function, function, Inventory, InventoryBuilder};

#[ffi_function]
fn do_math(x: u32) -> u32 {
    // Change this line, run `cargo build` and click `Hot Reload` in Unity
    x + 1
}

#[rustfmt::skip]
#[allow(unused)]
pub fn my_inventory() -> Inventory {
    InventoryBuilder::new()
        .register(function!(do_math))
        .validate()
        .inventory()
}

#[test]
fn bindings_csharp() {
    use interoptopus::util::NamespaceMappings;
    use interoptopus::Interop;
    use interoptopus_backend_csharp::{Config, Generator};

    let inventory = my_inventory();
    let config = Config {
        class: "InteropClass".to_string(),
        dll_name: "unity_hot_reload".to_string(),
        namespace_mappings: NamespaceMappings::new("My.Company"),
        ..Config::default()
    };

    Generator::new(config, inventory).write_file("unity/Assets/MyRustLib/Interop.cs").unwrap();
}
