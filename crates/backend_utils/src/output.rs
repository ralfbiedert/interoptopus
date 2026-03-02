use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Default)]
pub struct Multibuf {
    buffers: HashMap<String, String>,
}

impl Multibuf {
    pub fn new() -> Multibuf {
        Default::default()
    }

    pub fn add_buffer(&mut self, name: impl AsRef<str>, value: String) {
        self.buffers.insert(name.as_ref().to_string(), value);
    }

    pub fn buffer(&self, name: &str) -> Option<&String> {
        self.buffers.get(name)
    }

    pub fn write_buffer(&self, path: &str) -> Result<(), std::io::Error> {
        let path = Path::new(path);
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        let content = self.buffers.get(name).ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotFound, format!("buffer '{}' not found", name))
        })?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.buffers.iter()
    }
}
