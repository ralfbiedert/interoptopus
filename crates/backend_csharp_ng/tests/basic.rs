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
