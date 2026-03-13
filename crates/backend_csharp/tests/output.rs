use interoptopus::inventory::RustInventory;
use interoptopus_csharp::dispatch::Dispatch;
use interoptopus_csharp::lang::meta::FileEmission;
use interoptopus_csharp::output::FileName;
use interoptopus_csharp::RustLibrary;
use std::error::Error;

#[test]
fn output() -> Result<(), Box<dyn Error>> {
    let inventory = RustInventory::new();
    let multibuf = RustLibrary::builder(inventory)
        .dispatch(Dispatch::custom(|x, _| match x.emission {
            FileEmission::Common => FileName::new("Interop.Common.cs"),
            FileEmission::Module(_) => FileName::new("Interop.cs"),
        }))
        .build()
        .process()?;

    for output in multibuf.iter() {
        println!("{output:?}");
    }

    Ok(())
}
