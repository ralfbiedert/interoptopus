use crate::Error;
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;

/// If `dotnet` is installed, run the command as `dotnet command` from `path`, ignore and succeed otherwise.
pub fn run_dotnet_if_installed(path: impl AsRef<Path>, command: &str) -> Result<String, Error> {
    let child = match Command::new("dotnet").arg(command).current_dir(path).spawn() {
        Ok(x) => x,
        Err(x @ std::io::Error { .. }) if x.kind() == ErrorKind::NotFound => {
            return Ok("dotnet not found, skipped".to_string());
        }
        Err(x) => return Err(Error::DotNetCliCommandFailed(x.to_string())),
    };

    let output = child
        .wait_with_output()
        .map_err(|arg0: std::io::Error| Error::DotNetCliCommandFailed(arg0.to_string()))?;

    if output.status.success() {
        Ok(output.status.to_string())
    } else {
        Err(Error::DotNetCliCommandFailed("command failed with no output available".to_string()))
    }
}
