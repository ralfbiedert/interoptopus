use interoptopus::Error;
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;

/// If `python` is installed, run the given file from `path`, ignore and succeed otherwise.
pub fn run_python_if_installed<P: AsRef<Path>>(path: P, file: &str) -> Result<String, Error> {
    let child = match Command::new("/c/Users/rb/.miniconda3/python").arg(file).current_dir(path).spawn() {
        Ok(x) => x,
        Err(x @ std::io::Error { .. }) if x.kind() == ErrorKind::NotFound => {
            return Ok("Python not found, skipped".to_string());
        }
        Err(x) => return Err(Error::IO(x)),
    };

    let output = child.wait_with_output()?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        let stdout = String::from_utf8(output.stdout)?;
        let stderr = String::from_utf8(output.stderr)?;
        Err(Error::TestFailed(stdout, stderr))
    }
}
