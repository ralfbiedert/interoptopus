// use interoptopus::util::NamespaceMappings;
// use interoptopus::Error;
// use interoptopus::Interop;
//
// #[test]
// #[cfg_attr(miri, ignore)]
// fn bindings_csharp() -> Result<(), Error> {
//     use interoptopus_backend_csharp::run_dotnet_command_if_installed;
//     use interoptopus_backend_csharp::{Config, Generator};
//
//     Generator::new(
//         Config {
//             class: "InteropClass".to_string(),
//             dll_name: "example_complex".to_string(),
//             namespace_mappings: NamespaceMappings::new("My.Company"),
//             ..Config::default()
//         },
//         example_complex::ffi_inventory(),
//     )
//     .write_file("bindings/csharp/Interop.cs")?;
//
//     run_dotnet_command_if_installed("bindings/csharp", "test")?;
//
//     Ok(())
// }
//
// #[test]
// #[cfg_attr(miri, ignore)]
// fn bindings_c() -> Result<(), Error> {
//     use interoptopus_backend_c::{CDocumentationStyle, Config, Generator};
//
//     let custom_defines = r"
// // Custom attribute.
// #define __FUNCTION_ATTR __declspec( dllimport )
//     "
//     .to_string();
//
//     Generator::new(
//         Config {
//             ifndef: "example_complex".to_string(),
//             // Add an unneeded include for testing purposes.
//             additional_includes: vec!["<stdio.h>".into()],
//             function_attribute: "__FUNCTION_ATTR ".to_string(),
//             custom_defines,
//             documentation: CDocumentationStyle::Inline,
//             ..Config::default()
//         },
//         example_complex::ffi_inventory(),
//     )
//     .write_file("bindings/c/example_complex.h")?;
//
//     Ok(())
// }
//
// #[test]
// #[cfg_attr(miri, ignore)]
// fn bindings_cpython_cffi() -> Result<(), Error> {
//     use interoptopus_backend_cpython::run_python_if_installed;
//     use interoptopus_backend_cpython::{Config, Generator};
//
//     Generator::new(Config::default(), example_complex::ffi_inventory()).write_file("bindings/python/example_complex.py")?;
//
//     run_python_if_installed("bindings/python/", "app.py")?;
//
//     Ok(())
// }
