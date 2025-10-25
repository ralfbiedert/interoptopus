use backend_csharp_ng::dispatch::Dispatch;
use backend_csharp_ng::stage::output_director;
use backend_csharp_ng::{RustPlugin, RustPluginConfig};
use interoptopus::inventory::Inventory;

#[test]
fn rust_plugin() {
    let inventory = Inventory::new();
    let _ = RustPlugin::new(inventory).process();
}

#[test]
fn rust_plugin_builder() {
    let inventory = Inventory::new();
    let _ = RustPlugin::builder(inventory).build();
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
    // let dispatch = Dispatch::single_file();
    // let dispatch = Dispatch::custom(|x| match x {
    //     ... => "foo.cs",
    //     ... => "builtins.cs",
    //     ... => "build.csproj"
    // });

    // let dispatch = Dispatch::custom(|x| "foo".to_string());
    let dispatch = Dispatch::single_file();

    let config = RustPluginConfig { master_output: output_director::Config { ..Default::default() }, ..Default::default() };
    let inventory = Inventory::new();
    let output = RustPlugin::with_config(inventory, config).process();
    output.write_files();
}
