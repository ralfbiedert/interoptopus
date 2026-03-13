#[macro_use]
mod common;

mod reference_project {
    use interoptopus_csharp::RustLibrary;

    #[test]
    fn prerequisites() -> Result<(), Box<dyn std::error::Error>> {
        RustLibrary::builder(reference_project::inventory())
            .dll_name("foo")
            .build()
            .process()?
            .write_buffers_to("tests/reference_project")?;

        Ok(())
    }
}

mod model {
    mod service_rval_result;
}
