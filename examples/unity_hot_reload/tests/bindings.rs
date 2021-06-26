use interoptopus::testing::csharp::run_dotnet_command_if_installed;
use interoptopus::util::NamespaceMappings;
use interoptopus::{Error, Interop};
use interoptopus_backend_csharp::unity::{Assembly, UnityReloadHelper};

#[test]
#[cfg_attr(miri, ignore)]
fn bindings_csharp() -> Result<(), Error> {
    use interoptopus_backend_csharp::{Config, Generator};

    let inventory = unity_hot_reload::ffi_inventory();
    let config = Config {
        class: "InteropClass".to_string(),
        dll_name: "unity_hot_reload".to_string(),
        namespace_mappings: NamespaceMappings::new("My.Company"),
        ..Config::default()
    };

    Generator::new(config.clone(), inventory.clone()).write_file("bindings/csharp/Interop.cs")?;

    // run_dotnet_command_if_installed("bindings/csharp", "test")?;

    let reload_helper = UnityReloadHelper {
        inventory,
        config,
        assembly: Assembly::Debug,
        target_path_hint: "../../target".to_string(),
        asset_name: "MyRustLib2".to_string(),
        interop_files: vec!["bindings/csharp/Interop.cs".to_string()],
    };

    reload_helper.write_to_asset_folder("unity_hot_reload/Assets/")?;

    Ok(())
}
