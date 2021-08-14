use interoptopus::util::NamespaceMappings;
use interoptopus::{Error, Interop};

#[test]
#[cfg_attr(miri, ignore)]
fn bindings_csharp() -> Result<(), Error> {
    use interoptopus_backend_csharp::{Config, Generator};

    Generator::new(
        Config {
            class: "InteropClass".to_string(),
            dll_name: "example_hello_world".to_string(),
            namespace_mappings: NamespaceMappings::new("My.Company"),
            ..Config::default()
        },
        example_hello_world::my_inventory(),
    )
    .write_file("bindings/csharp/Interop.cs")?;

    Ok(())
}

#[test]
#[cfg_attr(miri, ignore)]
fn bindings_c() -> Result<(), Error> {
    use interoptopus_backend_c::{Config, Generator};

    Generator::new(
        Config {
            ifndef: "example_hello_world".to_string(),
            ..Config::default()
        },
        example_hello_world::my_inventory(),
    )
    .write_file("bindings/c/example_complex.h")?;

    Ok(())
}

#[test]
#[cfg_attr(miri, ignore)]
fn bindings_cpython_cffi() -> Result<(), Error> {
    use interoptopus_backend_cpython_cffi::{Config, Generator};

    let library = example_hello_world::my_inventory();
    Generator::new(Config::default(), library).write_file("bindings/python/example_complex.py")?;

    Ok(())
}
