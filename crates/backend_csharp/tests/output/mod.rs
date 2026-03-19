mod foo;

use interoptopus::inventory::RustInventory;
use interoptopus_csharp::dispatch::Dispatch;
use interoptopus_csharp::lang::meta::FileEmission;
use interoptopus_csharp::output::Target;
use interoptopus_csharp::RustLibrary;
use std::error::Error;

#[test]
fn output() -> Result<(), Box<dyn Error>> {
    let inventory = RustInventory::new();
    let multibuf = RustLibrary::builder(inventory)
        .dispatch(Dispatch::custom(|x, _| match x.emission {
            FileEmission::Common => Target::new("Interop.Common.cs", "My.Company.Common"),
            FileEmission::Default | FileEmission::CustomModule(_) => Target::new("Interop.cs", "My.Company"),
        }))
        .build()
        .process()?;

    for output in multibuf.iter() {
        println!("{output:?}");
    }

    Ok(())
}
