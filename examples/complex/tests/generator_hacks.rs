use interoptopus::writer::IndentWriter;
use interoptopus::Error;
use std::fs::File;

#[test]
fn generate_csharp() -> Result<(), Error> {
    use interoptopus_backend_csharp::{Config, Generator, Interop};

    let library = example_complex::ffi_inventory();

    let config = Config {
        namespace: "My.Company".to_string(),
        class: "InteropClass".to_string(),
        dll_name: "example_complex".to_string(),
        ..Config::default()
    };

    let generator = Generator::new(config, library);

    let mut file = File::create("bindings/csharp/Interop.cs")?;
    let mut writer = IndentWriter::new(&mut file);

    generator.write_to(&mut writer)?;

    Ok(())
}

#[test]
fn generate_c() -> Result<(), Error> {
    use interoptopus_backend_c::{Config, Generator, Interop};

    let custom_defines = r"
// Custom attribute.
#define __FUNCTION_ATTR __declspec( dllimport )
    "
    .to_string();

    let library = example_complex::ffi_inventory();

    let config = Config {
        ifndef: "example_complex".to_string(),
        function_attribute: "__FUNCTION_ATTR ".to_string(),
        custom_defines,
        ..Config::default()
    };

    let generator = Generator::new(config, library);

    let mut file = File::create("bindings/c/example_complex.h")?;
    let mut writer = IndentWriter::new(&mut file);

    generator.write_to(&mut writer)?;

    Ok(())
}

#[test]
fn generate_cpython_cffi() -> Result<(), Error> {
    use interoptopus_backend_cpython_cffi::{Config, Generator, Interop};

    let library = example_complex::ffi_inventory();
    let config = Config::default();

    let generator = Generator::new(config, library);

    let mut file = File::create("bindings/python/example_complex.py")?;
    let mut writer = IndentWriter::new(&mut file);

    generator.write_to(&mut writer)?;

    Ok(())
}
