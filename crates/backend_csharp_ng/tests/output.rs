use backend_csharp_ng::RustLibrary;
use interoptopus::inventory::Inventory;
use std::error::Error;

#[test]
fn output() -> Result<(), Box<dyn Error>> {
    let inventory = Inventory::new();
    let multibuf = RustLibrary::new(inventory).process()?;

    for output in multibuf.iter() {
        println!("{:?}", output);
    }

    Ok(())
}
