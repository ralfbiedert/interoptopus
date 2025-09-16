/// Doesn't exist in C, but other languages can benefit from accidentally using 'private' fields.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Visibility {
    Public,
    Private,
}

pub struct Namespace {
    name: String,
}

impl Namespace {
    #[must_use]
    pub fn new(name: String) -> Self {
        Self { name }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Markdown generated from the `///` you put on Rust code.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct Docs {
    pub lines: Vec<String>,
}

impl Docs {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn from_line(joined_line: &str) -> Self {
        if joined_line.is_empty() {
            Self::new()
        } else {
            Self { lines: joined_line.split('\n').map(std::string::ToString::to_string).collect() }
        }
    }

    #[must_use]
    pub const fn from_lines(lines: Vec<String>) -> Self {
        Self { lines }
    }
}
