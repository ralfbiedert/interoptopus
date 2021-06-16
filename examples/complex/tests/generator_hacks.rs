use interoptopus::generators::Interop;
use interoptopus::testing::c::compile_c_app_if_installed;
use interoptopus::testing::csharp::run_dotnet_command_if_installed;
use interoptopus::testing::python::run_python_if_installed;
use interoptopus::Error;

#[test]
fn bindings_csharp() -> Result<(), Error> {
    use interoptopus_backend_csharp::{Config, Generator};

    Generator::new(
        Config {
            namespace: "My.Company".to_string(),
            class: "InteropClass".to_string(),
            dll_name: "example_complex".to_string(),
            ..Config::default()
        },
        example_complex::ffi_inventory(),
    )
    .write_file("bindings/csharp/Interop.cs")?;

    run_dotnet_command_if_installed("bindings/csharp", "test")?;

    Ok(())
}

#[test]
fn bindings_c() -> Result<(), Error> {
    use interoptopus_backend_c::{Config, Generator};

    let custom_defines = r"
// Custom attribute.
#define __FUNCTION_ATTR __declspec( dllimport )
    "
    .to_string();

    Generator::new(
        Config {
            ifndef: "example_complex".to_string(),
            function_attribute: "__FUNCTION_ATTR ".to_string(),
            custom_defines,
            ..Config::default()
        },
        example_complex::ffi_inventory(),
    )
    .write_file("bindings/c/example_complex.h")?;

    compile_c_app_if_installed("bindings/c", "bindings/c/app.c")?;

    Ok(())
}

#[test]
fn bindings_cpython_cffi() -> Result<(), Error> {
    use interoptopus_backend_cpython_cffi::{Config, Generator};

    Generator::new(Config::default(), example_complex::ffi_inventory()).write_file("bindings/python/example_complex.py")?;

    run_python_if_installed("bindings/python/", "app.py")?;

    Ok(())
}
