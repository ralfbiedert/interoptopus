use backend_csharp_ng::RustPlugin;
use interoptopus::inventory::Inventory;
use std::error::Error;

#[test]
fn output() -> Result<(), Box<dyn Error>> {
    let inventory = Inventory::new();
    let multibuf = RustPlugin::new(inventory).process()?;

    for output in multibuf.iter() {
        println!("{:?}", output);
    }

    Ok(())
}
