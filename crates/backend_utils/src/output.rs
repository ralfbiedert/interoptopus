use std::collections::HashMap;

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

    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.buffers.iter()
    }
}
