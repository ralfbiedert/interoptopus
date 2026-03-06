use interoptopus::inventory::RustInventory;
use interoptopus_csharp::RustLibrary;
use std::error::Error;

#[test]
fn output() -> Result<(), Box<dyn Error>> {
    let inventory = RustInventory::new();
    let multibuf = RustLibrary::new(inventory).process()?;

    for output in multibuf.iter() {
        println!("{:?}", output);
    }

    Ok(())
}
