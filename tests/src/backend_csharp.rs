use anyhow::anyhow;
use interoptopus::backend::{NamespaceMappings, NAMESPACE_COMMON};
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;

pub fn common_namespace_mappings() -> NamespaceMappings {
    NamespaceMappings::new("My.Company").add(NAMESPACE_COMMON, "My.Company.Common")
}

/// If `dotnet` is installed, run the command as `dotnet command` from `path`, ignore and succeed otherwise.
pub fn run_dotnet_command_if_installed(path: impl AsRef<Path>, command: &str) -> Result<String, anyhow::Error> {
    let child = match Command::new("dotnet").arg(command).current_dir(path).spawn() {
        Ok(x) => x,
        Err(x @ std::io::Error { .. }) if x.kind() == ErrorKind::NotFound => {
            return Ok("dotnet not found, skipped".to_string());
        }
        Err(x) => return Err(anyhow!("dotnet command failed: {}", x)),
    };

    let output = child.wait_with_output()?;

    if output.status.success() {
        Ok(output.status.to_string())
    } else {
        Err(anyhow!("Test failed"))
    }
}

#[cfg(not(target_os = "windows"))]
pub fn compile_c_app_if_installed(_app: impl AsRef<Path>) -> Result<(), anyhow::Error> {
    Ok(())
}

// Not used!
#[macro_export]
macro_rules! compile_output_csharp {
    ($generated:expr) => {{
        if std::env::var($crate::UPDATE_BINDINGS).is_ok() {
            let temp_dir = $crate::tempdir()?;
            let header_file = temp_dir.path().join("header.h");
            let c_file = temp_dir.path().join("app.c");

            std::fs::write(header_file, $generated)?;
            std::fs::write(
                c_file.clone(),
                r#"
                #include "header.h"
                void main() {}
            "#,
            )?;
            $crate::backend_c::compile_c_app_if_installed(c_file)?;
        }
    }};
}
