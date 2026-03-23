//! Output file descriptors for generated C# code.

use interoptopus_backends::output::Overwrite;

/// A file name and namespace pair, e.g., `Interop.cs` in namespace `My.Company`.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Target {
    file_name: String,
    namespace: String,
    #[cfg_attr(not(test), allow(dead_code))]
    overwrite: Overwrite,
}

impl Target {
    /// Creates a new target with the given file name and C# namespace.
    ///
    /// The default overwrite policy is [`Overwrite::Always`].
    pub fn new(file_name: impl AsRef<str>, namespace: impl AsRef<str>) -> Self {
        Self { file_name: file_name.as_ref().to_string(), namespace: namespace.as_ref().to_string(), overwrite: Overwrite::Always }
    }

    /// Sets the overwrite policy for this target.
    #[must_use]
    pub fn overwrite(mut self, overwrite: Overwrite) -> Self {
        self.overwrite = overwrite;
        self
    }

    /// The output file name (e.g., `"Interop.cs"`).
    #[must_use]
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    /// The C# namespace for declarations in this file.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// The overwrite policy for this target.
    #[must_use]
    pub fn overwrite_policy(&self) -> Overwrite {
        self.overwrite
    }
}

/// A custom file type, e.g., `csproj`.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct CustomFileType(String);

/// The kind of file being generated.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum FileType {
    /// A `.cs` source file.
    Csharp,
    /// A non-C# file (e.g., `.csproj`).
    Custom(CustomFileType),
}

/// A concrete output file: target path plus file type.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Output {
    /// The file name and namespace.
    pub target: Target,
    /// The kind of file.
    pub kind: FileType,
}
