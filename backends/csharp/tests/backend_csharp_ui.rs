use interoptopus::testing::assert_file_matches_generated;
use interoptopus::util::NamespaceMappings;
use interoptopus::{Error, Interop};
use interoptopus_backend_csharp::overloads::{DotNet, Unity};
use interoptopus_backend_csharp::{
    run_dotnet_command_if_installed, CSharpVisibility, Config, DocConfig, DocGenerator, Generator, ParamSliceType, Unsafe, Unsupported, WriteTypes,
};
use std::path::{Path, PathBuf};
use tempdir::TempDir;

/// Writes a simple project config so `dotnet build` works.
pub fn write_simple_project_file(path: impl AsRef<Path>) -> Result<(), Error> {
    let csprj = r#"
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <TargetFramework>netcoreapp3.1</TargetFramework>
    <AllowUnsafeBlocks>true</AllowUnsafeBlocks>
  </PropertyGroup>
  <ItemGroup>
    <PackageReference Include="Microsoft.NET.Test.Sdk" Version="15.9.0" />
  </ItemGroup>
</Project>"#;

    let mut project_path = PathBuf::new();
    project_path.push(path);
    project_path.push("project.csproj");
    std::fs::write(project_path, csprj)?;
    Ok(())
}

/// Generates runnable bindings for the reference project.
fn generate_bindings_multi(folder: impl AsRef<Path>, use_unsafe: Unsafe, param_slice_type: ParamSliceType, config: Option<Config>) -> Result<(), Error> {
    let library = interoptopus_reference_project::ffi_inventory();
    let namespace_mappings = NamespaceMappings::new("My.Company").add("common", "My.Company.Common");

    let config = config.unwrap_or(Config {
        dll_name: "interoptopus_reference_project".to_string(),
        namespace_mappings,
        visibility_types: CSharpVisibility::AsDeclared,
        unsupported: Unsupported::Comment,
        param_slice_type,
        use_unsafe,
        ..Config::default()
    });

    for namespace_id in library.namespaces() {
        let file_name = format!("{}/Interop.{}.cs", folder.as_ref().to_str().ok_or(Error::FileNotFound)?, namespace_id).replace("..", ".");

        let write_types = if namespace_id.is_empty() {
            WriteTypes::Namespace
        } else {
            WriteTypes::NamespaceAndInteroptopusGlobal
        };

        let config = Config {
            namespace_id: namespace_id.clone(),
            write_types,
            ..config.clone()
        };

        let mut generator = Generator::new(config.clone(), interoptopus_reference_project::ffi_inventory());

        generator.add_overload_writer(DotNet::new());

        if use_unsafe.any_unsafe() {
            generator.add_overload_writer(Unity::new());
        }

        generator.write_file(file_name)?;
    }

    Ok(())
}

fn generate_documentation(output: &str) -> Result<(), Error> {
    let inventory = interoptopus_reference_project::ffi_inventory();
    let mut generator = Generator::new(
        Config {
            use_unsafe: Unsafe::UnsafePlatformMemCpy,
            ..Config::default()
        },
        inventory.clone(),
    );

    generator.add_overload_writer(DotNet::new()).add_overload_writer(Unity::new());

    DocGenerator::new(&inventory, &generator, DocConfig::default()).write_file(output)
}

fn generate_safe() -> Result<(), Error> {
    generate_bindings_multi("tests/output_safe", Unsafe::None, ParamSliceType::Array, None)
}

fn generate_unsafe() -> Result<(), Error> {
    generate_bindings_multi("tests/output_unsafe", Unsafe::UnsafePlatformMemCpy, ParamSliceType::Span, None)
}

#[test]
#[cfg_attr(miri, ignore)]
fn bindings_match_reference() -> Result<(), Error> {
    generate_safe()?;
    generate_unsafe()?;
    generate_bindings_multi("tests/output_unity/Assets", Unsafe::UnsafePlatformMemCpy, ParamSliceType::Array, None)?;

    assert_file_matches_generated("tests/output_safe/Interop.cs");
    assert_file_matches_generated("tests/output_safe/Interop.common.cs");

    assert_file_matches_generated("tests/output_unsafe/Interop.cs");
    assert_file_matches_generated("tests/output_unsafe/Interop.common.cs");

    assert_file_matches_generated("tests/output_unity/Assets/Interop.cs");
    assert_file_matches_generated("tests/output_unity/Assets/Interop.common.cs");

    generate_documentation("tests/output/reference_project.md")?;

    Ok(())
}

#[test]
#[cfg_attr(miri, ignore)]
fn bindings_work() -> Result<(), Error> {
    generate_safe()?;
    generate_unsafe()?;

    run_dotnet_command_if_installed("tests/output_safe/", "test")?;
    run_dotnet_command_if_installed("tests/output_unsafe/", "test")?;
    Ok(())
}

#[test]
#[cfg_attr(miri, ignore)]
fn prepare_benchmarks() -> Result<(), Error> {
    generate_bindings_multi("benches", Unsafe::UnsafePlatformMemCpy, ParamSliceType::Array, None)?;
    Ok(())
}

#[test]
#[cfg_attr(miri, ignore)]
fn config_rename_symbols() -> Result<(), Error> {
    let temp = TempDir::new("interoptopus_csharp")?;

    let config = Config {
        namespace_mappings: NamespaceMappings::new("My.Company").add("common", "My.Company.Common"),
        unsupported: Unsupported::Comment,
        rename_symbols: true,
        use_unsafe: Unsafe::UnsafeKeyword,
        ..Config::default()
    };

    generate_bindings_multi(temp.path(), Unsafe::None, ParamSliceType::Array, Some(config))?;
    write_simple_project_file(temp.path())?;
    run_dotnet_command_if_installed(temp.path(), "build")?;

    Ok(())
}

#[test]
#[cfg_attr(miri, ignore)]
fn config_no_unsafe() -> Result<(), Error> {
    let temp = TempDir::new("interoptopus_csharp")?;

    let config = Config {
        namespace_mappings: NamespaceMappings::new("My.Company").add("common", "My.Company.Common"),
        unsupported: Unsupported::Comment,
        use_unsafe: Unsafe::None,
        ..Config::default()
    };

    generate_bindings_multi(temp.path(), Unsafe::None, ParamSliceType::Array, Some(config))?;
    write_simple_project_file(temp.path())?;
    run_dotnet_command_if_installed(temp.path(), "build")?;

    Ok(())
}

#[test]
#[cfg_attr(miri, ignore)]
fn config_unsafe_memcpy() -> Result<(), Error> {
    let temp = TempDir::new("interoptopus_csharp")?;

    let config = Config {
        namespace_mappings: NamespaceMappings::new("My.Company").add("common", "My.Company.Common"),
        unsupported: Unsupported::Comment,
        use_unsafe: Unsafe::UnsafePlatformMemCpy,
        ..Config::default()
    };

    generate_bindings_multi(temp.path(), Unsafe::None, ParamSliceType::Array, Some(config))?;
    write_simple_project_file(temp.path())?;
    run_dotnet_command_if_installed(temp.path(), "build")?;

    Ok(())
}

#[test]
#[cfg_attr(miri, ignore)]
fn config_visibility_force_visibility_internal() -> Result<(), Error> {
    let temp = TempDir::new("interoptopus_csharp")?;

    let config = Config {
        namespace_mappings: NamespaceMappings::new("My.Company").add("common", "My.Company.Common"),
        use_unsafe: Unsafe::UnsafeKeyword,
        unsupported: Unsupported::Comment,
        visibility_types: CSharpVisibility::ForceInternal,
        ..Config::default()
    };

    generate_bindings_multi(temp.path(), Unsafe::None, ParamSliceType::Array, Some(config))?;
    write_simple_project_file(temp.path())?;
    run_dotnet_command_if_installed(temp.path(), "build")?;

    Ok(())
}

#[test]
#[cfg_attr(miri, ignore)]
fn config_visibility_force_visibility_public() -> Result<(), Error> {
    let temp = TempDir::new("interoptopus_csharp")?;

    let config = Config {
        namespace_mappings: NamespaceMappings::new("My.Company").add("common", "My.Company.Common"),
        use_unsafe: Unsafe::UnsafeKeyword,
        unsupported: Unsupported::Comment,
        visibility_types: CSharpVisibility::ForcePublic,
        ..Config::default()
    };

    generate_bindings_multi(temp.path(), Unsafe::None, ParamSliceType::Array, Some(config))?;
    write_simple_project_file(temp.path())?;
    run_dotnet_command_if_installed(temp.path(), "build")?;

    Ok(())
}
