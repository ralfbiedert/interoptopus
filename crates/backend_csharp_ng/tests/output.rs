use backend_csharp_ng::RustPlugin;
use backend_csharp_ng::dispatch::Dispatch;
use interoptopus::inventory::Inventory;

#[test]
fn output() {
    let inventory = Inventory::new();
    let dispatch = Dispatch::single_file();
    let output = RustPlugin::builder(inventory).dispatch(dispatch).build().process();
}
