use std::path::Path;
use crate::Error;
use std::process::{Command};
use std::io::ErrorKind;

/// If `dotnet` is installed, run the command as `dotnet command` from `path`, ignore and succeed otherwise.
pub fn run_dotnet_command_if_installed<P: AsRef<Path>>(path: P, command: &str) -> Result<String, Error> {
    let child = match Command::new("dotnet").arg(command).current_dir(path).spawn() {
        Ok(x) => x,
        Err(x @ std::io::Error { .. }) if x.kind() == ErrorKind::NotFound => { return Ok("dotnet not found, skipped".to_string()); },
        Err(x) => { return Err(Error::IO(x)) }
    };

    let output = child.wait_with_output()?;

    if output.status.success() {
        Ok(output.status.to_string())
    } else {
        Err(Error::TestFailed)
    }
}