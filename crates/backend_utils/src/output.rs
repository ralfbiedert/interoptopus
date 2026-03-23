//! Multi-file output buffering for code generation.

use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::Path;

/// Controls whether a generated file overwrites an existing file on disk.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Overwrite {
    /// Always overwrite the file, even if it already exists.
    #[default]
    Always,
    /// Never overwrite; skip writing if the file already exists.
    Never,
}

/// A single output buffer entry with its content and write policy.
#[derive(Debug, Clone)]
struct BufEntry {
    content: String,
    overwrite: Overwrite,
}

/// A collection of named output buffers, typically one per generated file.
///
/// Backends build up a `Multibuf` during code generation, then write all buffers
/// to disk at the end via [`write_buffers_to`](Multibuf::write_buffers_to).
#[derive(Debug, Default)]
pub struct Multibuf {
    buffers: HashMap<String, BufEntry>,
}

impl Multibuf {
    /// Creates an empty `Multibuf`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds or replaces a named buffer with the given content.
    ///
    /// The default overwrite policy is [`Overwrite::Always`].
    pub fn add_buffer(&mut self, name: impl AsRef<str>, value: String) {
        self.buffers.insert(name.as_ref().to_string(), BufEntry { content: value, overwrite: Overwrite::Always });
    }

    /// Adds or replaces a named buffer with the given content and overwrite policy.
    pub fn add_buffer_with_overwrite(&mut self, name: impl AsRef<str>, value: String, overwrite: Overwrite) {
        self.buffers.insert(name.as_ref().to_string(), BufEntry { content: value, overwrite });
    }

    /// Sets the overwrite policy for an existing buffer. No-op if the buffer doesn't exist.
    pub fn set_overwrite(&mut self, name: &str, overwrite: Overwrite) {
        if let Some(entry) = self.buffers.get_mut(name) {
            entry.overwrite = overwrite;
        }
    }

    /// Returns the content of the named buffer, if it exists.
    #[must_use]
    pub fn buffer(&self, name: &str) -> Option<&String> {
        self.buffers.get(name).map(|e| &e.content)
    }

    /// Writes the named buffer to the current directory.
    pub fn write_buffer(&self, name: &str) -> Result<(), std::io::Error> {
        self.write_buffer_to(".", name)
    }

    /// Writes the named buffer to a destination path, joining `dir` and `name`.
    pub fn write_buffer_to(&self, dir: impl AsRef<Path>, name: &str) -> Result<(), std::io::Error> {
        let entry = self
            .buffers
            .get(name)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, format!("buffer '{name}' not found")))?;

        let path = dir.as_ref().join(name);
        if entry.overwrite == Overwrite::Never && path.exists() {
            return Ok(());
        }
        fs::write(path, &entry.content)?;
        Ok(())
    }

    /// Writes all buffers into the given directory, honoring each buffer's overwrite policy.
    pub fn write_buffers_to(&self, dir: impl AsRef<Path>) -> Result<(), std::io::Error> {
        let dir = dir.as_ref();
        for (name, entry) in &self.buffers {
            let path = dir.join(name);
            if entry.overwrite == Overwrite::Never && path.exists() {
                continue;
            }
            fs::write(path, &entry.content)?;
        }
        Ok(())
    }

    /// Iterates over all `(name, content)` pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.buffers.iter().map(|(k, v)| (k, &v.content))
    }
}

impl fmt::Display for Multibuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut keys: Vec<_> = self.buffers.keys().collect();
        keys.sort();
        for name in keys {
            writeln!(f, "=== {name} ===")?;
            write!(f, "{}", self.buffers[name].content)?;
        }
        Ok(())
    }
}
