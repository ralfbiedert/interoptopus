#[macro_use]
mod common;

mod reference_project {
    use interoptopus_csharp::dispatch::Dispatch;
    use interoptopus_csharp::lang::meta::FileEmission;
    use interoptopus_csharp::output::Target;
    use interoptopus_csharp::RustLibrary;

    #[test]
    fn prerequisites() -> Result<(), Box<dyn std::error::Error>> {
        RustLibrary::builder(reference_project::inventory())
            .dll_name("foo")
            .dispatch(Dispatch::custom(|x, _| match x.emission {
                FileEmission::Common => Target::new("Interop.Common.cs", "My.Company.Common"),
                FileEmission::Default => Target::new("Interop.cs", "My.Company"),
                FileEmission::CustomModule(_) => Target::new("Interop.cs", "My.Company"),
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
