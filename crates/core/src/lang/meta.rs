/// Doesn't exist in C, but other languages can benefit from accidentally using 'private' fields.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Visibility {
    Public,
    Private,
}

/// Additional information for user-defined types.
#[derive(Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Meta {
    docs: Docs,
    module: String,
}

impl Meta {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub const fn with_module_docs(module: String, docs: Docs) -> Self {
        Self { docs, module }
    }

    #[must_use]
    pub const fn with_docs(docs: Docs) -> Self {
        Self::with_module_docs(String::new(), docs)
    }

    #[must_use]
    pub const fn docs(&self) -> &Docs {
        &self.docs
    }

    #[must_use]
    pub fn module(&self) -> &str {
        &self.module
    }

    /// Convenience method used in generators
    #[must_use]
    pub fn is_module(&self, module: &str) -> bool {
        self.module == module
    }
}

/// Markdown generated from the `///` you put on Rust code.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct Docs {
    lines: Vec<String>,
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

    #[must_use]
    pub fn lines(&self) -> &[String] {
        &self.lines
    }
}
