use std::collections::HashMap;

#[derive(Default)]
pub struct Multibuf {
    buffers: HashMap<String, String>,
}
