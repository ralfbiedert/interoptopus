#[macro_use]
mod common;

mod reference_project {
    use interoptopus_csharp::dispatch::Dispatch;
    use interoptopus_csharp::lang::meta::FileEmission;
    use interoptopus_csharp::output::FileName;
    use interoptopus_csharp::RustLibrary;

    #[test]
    fn prerequisites() -> Result<(), Box<dyn std::error::Error>> {
        RustLibrary::builder(reference_project::inventory())
            .dll_name("foo")
            .dispatch(Dispatch::custom(|x, _| match x.emission {
                FileEmission::Common => FileName::new("Interop.Common.cs"),
                FileEmission::Default => FileName::new("Interop.cs"),
                FileEmission::CustomModule(_) => FileName::new("Interop.cs"),
            }))
            .build()
            .process()?
            .write_buffers_to("tests/reference_project")?;

        Ok(())
    }
}

mod model {
    mod service_rval_result;
}
