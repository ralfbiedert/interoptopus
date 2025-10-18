pub struct Lines {
    lines: Vec<String>,
}

pub enum Section {
    Lines(Lines),
    Nested(Box<Section>),
}

pub struct Paragraph {
    section: Vec<Section>,
}
