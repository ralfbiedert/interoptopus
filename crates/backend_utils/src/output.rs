use std::collections::HashMap;

pub struct Lines {
    lines: Vec<String>,
}

pub enum Section {
    Lines(Lines),
    Nested(Box<Section>),
}

pub struct Buffer {
    section: Vec<Section>,
}

#[derive(Default)]
pub struct Multibuf {
    buffers: HashMap<String, Buffer>,
}
