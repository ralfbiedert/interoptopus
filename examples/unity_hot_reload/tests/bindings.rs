use interoptopus::util::NamespaceMappings;
use interoptopus::{Error, Interop};

#[test]
#[cfg_attr(miri, ignore)]
fn bindings_csharp() -> Result<(), Error> {
    use interoptopus_backend_csharp::{Config, Generator};

    let inventory = unity_hot_reload::my_inventory();
    let config = Config {
        class: "InteropClass".to_string(),
        dll_name: "unity_hot_reload".to_string(),
        namespace_mappings: NamespaceMappings::new("My.Company"),
        ..Config::default()
    };

    Generator::new(config, inventory).write_file("bindings/csharp/Interop.cs")?;

    Ok(())
}
