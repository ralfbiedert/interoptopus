/// Doesn't exist in C, but other languages can benefit from accidentally using 'private' fields.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Visibility {
    Public,
    Private,
}

/// Additional information for user-defined types.
#[derive(Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Meta {
    documentation: Documentation,
    namespace: String,
}

impl Meta {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub const fn with_namespace_documentation(namespace: String, documentation: Documentation) -> Self {
        Self { documentation, namespace }
    }

    #[must_use]
    pub const fn with_documentation(documentation: Documentation) -> Self {
        Self::with_namespace_documentation(String::new(), documentation)
    }

    #[must_use]
    pub const fn documentation(&self) -> &Documentation {
        &self.documentation
    }

    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Convenience method used in generators
    #[must_use]
    pub fn is_namespace(&self, namespace: &str) -> bool {
        self.namespace == namespace
    }
}

/// Markdown generated from the `///` you put on Rust code.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct Documentation {
    lines: Vec<String>,
}

impl Documentation {
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
