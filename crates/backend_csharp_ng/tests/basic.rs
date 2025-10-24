use backend_csharp_ng::stage::master_output;
use backend_csharp_ng::{RustPlugin, RustPluginConfig};
use interoptopus::inventory::Inventory;

#[test]
fn can_run_pipeline() {
    let config = RustPluginConfig {
        type_id_mapping: Default::default(),
        type_id_mapping2: Default::default(),
        type_id_mapping3: Default::default(),
        type_id_mapping4: Default::default(),
        master_output: master_output::Config { ..Default::default() },
        ..Default::default()
    };
    let inventory = Inventory::new();
    let _ = RustPlugin::with_config(inventory, config).process();
}

#[test]
fn output() {
    // todo: would allow to specify same types (e.g., builtins) multiple times
    // let output = Output::new()
    //     .output("foo.cs", File::builtins())
    //     .output("bar.cs", File::user())
    //     .output("ada.cs", File::custom())
    //     .output("asda.csproj", File::build());

    // todo: super weird to specify
    // let output_dispatch = Dispatch::new()
    //     .types(|ty| if ty == 0x0 { "foo.cs" } else { "bar.cs" })
    //     .fns(|fns| "bar.cs")
    //     .build(|b| "bar.cs");

    // this is it?!
    let dispatch = Dispatch::single_file();
    let dispatch = Dispatch::custom(|x| match x {
        ... => "foo.cs",
        ... => "builtins.cs",
        ... => "build.csproj"
    });

    let config = RustPluginConfig { master_output: master_output::Config { ..Default::default() }, ..Default::default() };
    let inventory = Inventory::new();
    let output = RustPlugin::with_config(inventory, config).process();
    output.write_files();
}
