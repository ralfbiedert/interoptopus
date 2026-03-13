/// A file name, e.g., `Interop.Foo.Bar.cs`.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Target {
    file_name: String,
    namespace: String,
}

impl Target {
    pub fn new(file_name: impl AsRef<str>, namespace: impl AsRef<str>) -> Self {
        Self { file_name: file_name.as_ref().to_string(), namespace: namespace.as_ref().to_string() }
    }

    #[must_use]
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }
}

/// A custom file type, e.g., `csproj`.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct CustomFileType(String);

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum FileType {
    Csharp,
    Custom(CustomFileType),
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Output {
    pub target: Target,
    pub kind: FileType,
}
