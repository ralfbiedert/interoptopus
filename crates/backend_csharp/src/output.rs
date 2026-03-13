/// A file name, e.g., `Interop.Foo.Bar.cs`.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct FileName(String);

impl FileName {
    pub fn new(name: impl AsRef<str>) -> Self {
        Self(name.as_ref().to_string())
    }
}

impl AsRef<str> for FileName {
    fn as_ref(&self) -> &str {
        &self.0
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
    pub name: FileName,
    pub kind: FileType,
}
