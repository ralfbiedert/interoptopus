use interoptopus::lang::meta::FileEmission;
use interoptopus_csharp::RustLibrary;
use interoptopus_csharp::config::{DllImportSearchPath, HeaderConfig, SearchPathConfig};
use interoptopus_csharp::dispatch::Dispatch;
use interoptopus_csharp::output::Target;

#[test]
fn interop() -> Result<(), Box<dyn std::error::Error>> {
    let multibuf = RustLibrary::builder(reference_project::inventory())
        .dll_name("reference_project")
        .dispatch(Dispatch::custom(|x, _| match x.emission {
            FileEmission::Common => Target::new("Interop.Common.cs", "My.Company.Common"),
            FileEmission::Default => Target::new("Interop.cs", "My.Company"),
            FileEmission::CustomModule(_) => Target::new("Interop.cs", "My.Company"),
        }))
        .header_config(HeaderConfig { emit_version: false })
        .search_path_config(SearchPathConfig { import_search_path: DllImportSearchPath::None })
        .build()
        .process()?;

    multibuf.write_buffers_to("tests/reference_project/Bindings")?;
    multibuf.write_buffers_to("benches/dotnet")?;

    // insta::assert_snapshot!(multibuf);

    Ok(())
}
