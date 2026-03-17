//! Multi-file output buffering for code generation.

use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::Path;

/// A collection of named output buffers, typically one per generated file.
///
/// Backends build up a `Multibuf` during code generation, then write all buffers
/// to disk at the end via [`write_buffers_to`](Multibuf::write_buffers_to).
#[derive(Debug, Default)]
pub struct Multibuf {
    buffers: HashMap<String, String>,
}

impl Multibuf {
    /// Creates an empty `Multibuf`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds or replaces a named buffer with the given content.
    pub fn add_buffer(&mut self, name: impl AsRef<str>, value: String) {
        self.buffers.insert(name.as_ref().to_string(), value);
    }

    /// Returns the content of the named buffer, if it exists.
    #[must_use]
    pub fn buffer(&self, name: &str) -> Option<&String> {
        self.buffers.get(name)
    }

    /// Writes the named buffer to the current directory.
    pub fn write_buffer(&self, name: &str) -> Result<(), std::io::Error> {
        self.write_buffer_to(".", name)
    }

    /// Writes the named buffer to a destination path, joining `dir` and `name`.
    pub fn write_buffer_to(&self, dir: impl AsRef<Path>, name: &str) -> Result<(), std::io::Error> {
        let content = self
            .buffers
            .get(name)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, format!("buffer '{name}' not found")))?;
        fs::write(dir.as_ref().join(name), content)?;
        Ok(())
    }

    /// Writes all buffers into the given directory.
    pub fn write_buffers_to(&self, dir: impl AsRef<Path>) -> Result<(), std::io::Error> {
        let dir = dir.as_ref();
        for (name, content) in &self.buffers {
            fs::write(dir.join(name), content)?;
        }
        Ok(())
    }

    /// Iterates over all `(name, content)` pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.buffers.iter()
    }
}

impl fmt::Display for Multibuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut keys: Vec<_> = self.buffers.keys().collect();
        keys.sort();
        for name in keys {
            writeln!(f, "=== {name} ===")?;
            write!(f, "{}", self.buffers[name])?;
        }
        Ok(())
    }
}
