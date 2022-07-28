use interoptopus::testing::assert_file_matches_generated;
use interoptopus::Error;
use interoptopus::Interop;
use interoptopus_backend_c::{compile_c_app_if_installed, CDocumentationStyle, CIndentationStyle, CNamingStyle, Config, Generator};
use std::path::Path;

fn nodocs_config() -> Config {
    Config {
        prefix: "my_library_".to_string(),
        documentation: CDocumentationStyle::None,
        ..Config::default()
    }
}

fn docs_inline_config() -> Config {
    Config {
        prefix: "my_library_".to_string(),
        documentation: CDocumentationStyle::Inline,
        indentation: CIndentationStyle::Allman,
        type_naming: CNamingStyle::SnakeCase,
        function_parameter_naming: CNamingStyle::SnakeCase,
        enum_variant_naming: CNamingStyle::ShoutySnakeCase,
        const_naming: CNamingStyle::ShoutySnakeCase,
        ..Config::default()
    }
}

fn generate_bindings_multi(folder: impl AsRef<Path>, config: Option<Config>) -> Result<(), Error> {
    let config = config.unwrap_or_default();

    let file_name = format!("{}/my_header.h", folder.as_ref().to_str().ok_or(Error::FileNotFound)?).replace("..", ".");

    let inventory = interoptopus_reference_project::ffi_inventory();

    let generator = Generator::new(config, inventory);
    generator.write_file(file_name)?;

    Ok(())
}

// fn generate_documentation(output: &str, config: Option<Config>) -> Result<(), Error> {
//     let config = config.unwrap_or(Config::default());
//     let inventory = interoptopus_reference_project::ffi_inventory();
//     let generator = Generator::new(config, inventory);
//     let doc_gen = DocGenerator::new(library, generator);
//     doc_gen.write_file(format!("{}/my_header.md", output))?;
//
//     Ok(())
// }

#[test]
#[cfg_attr(miri, ignore)]
fn bindings_match_reference() -> Result<(), Error> {
    generate_bindings_multi("tests/output_nodocs/", Some(nodocs_config()))?;
    generate_bindings_multi("tests/output_docs_inline/", Some(docs_inline_config()))?;

    assert_file_matches_generated("tests/output_nodocs/my_header.h");
    assert_file_matches_generated("tests/output_docs_inline/my_header.h");

    Ok(())
}

#[test]
#[cfg_attr(miri, ignore)]
fn bindings_work() -> Result<(), Error> {
    generate_bindings_multi("tests/output_nodocs/", Some(nodocs_config()))?;
    generate_bindings_multi("tests/output_docs_inline/", Some(docs_inline_config()))?;

    compile_c_app_if_installed("tests/output_nodocs/", "tests/output_nodocs/app.c")?;
    compile_c_app_if_installed("tests/output_docs_inline/", "tests/output_docs_inline/app.c")?;
    Ok(())
}
