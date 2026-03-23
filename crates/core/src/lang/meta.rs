//! Cross-cutting metadata: documentation, visibility, and emission/placement rules.

use std::borrow::Cow;

/// The visibility of an item when written. Not all backends support all visibility levels.
#[derive(Clone, Copy, Default, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Visibility {
    #[default]
    Public,
    Private,
}

/// A module identifier, e.g., `foo.bar`.
///
/// This does not have any semantic meaning beyond it being used to map
/// a module string (`foo.bar`) to a file name (e.g., `Interop.Foo.Bar.cs`)
/// at a later stage.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Module(Cow<'static, str>);

impl Module {
    /// Creates a module identifier from a string (e.g., `"foo.bar"`).
    #[must_use]
    pub fn from_string(s: impl Into<String>) -> Self {
        Self(Cow::from(s.into()))
    }

    /// Creates a module identifier from a static 'str (e.g., `"foo.bar"`).
    #[must_use]
    pub const fn from_str(s: &'static str) -> Self {
        Self(Cow::Borrowed(s))
    }
}

/// Items that should actually go to some file.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FileEmission {
    /// This is a 'common' type (like `Slice<u8>`) that needs to be emitted in some interop file,
    /// is not a builtin, but not specific to any customer project.
    Common,
    /// Variants for which the user never specified anything.
    Default,
    /// The type should be placed in the given module / file.
    CustomModule(Module),
}

/// Where an item definition should be placed in generated files.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Emission {
    /// This is a built-in type (e.g., `f32` <-> `float`) and does not need to be defined (emitted
    /// in generated interop code). Also used for "std" like builtins, `String` <-> `string`.
    Builtin,
    /// Items that should actually go to some file.
    FileEmission(FileEmission),
    /// The backend will _use_ the type as if it were auto-generated, but it is up to the user
    /// to actually provide it. Its definition will not be emitted.
    External,
}

impl Emission {
    /// Returns the `FileEmission` if this item should be written to a file.
    #[must_use]
    pub fn file_emission(&self) -> Option<&FileEmission> {
        match self {
            Self::FileEmission(fe) => Some(fe),
            _ => None,
        }
    }
}

/// Markdown generated from the `///` you put on Rust code.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Docs {
    pub lines: Vec<String>,
}

impl Docs {
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn from_line(joined_line: &str) -> Self {
        if joined_line.is_empty() {
            Self::empty()
        } else {
            Self { lines: joined_line.split('\n').map(std::string::ToString::to_string).collect() }
        }
    }

    #[must_use]
    pub const fn from_lines(lines: Vec<String>) -> Self {
        Self { lines }
    }
}

/// Given a set of inner-type emissions, determines the emission for a composite type.
///
/// Returns `Common` if all inner types are builtins or common, `CustomModule` if any
/// inner type has a custom module, and `Default` otherwise.
#[must_use]
pub fn common_or_module_emission(x: &[Emission]) -> Emission {
    if x.iter().all(|x| matches!(x, Emission::Builtin | Emission::FileEmission(FileEmission::Common))) {
        Emission::FileEmission(FileEmission::Common)
    } else if x.iter().any(|x| matches!(x, Emission::FileEmission(FileEmission::CustomModule(_)))) {
        Emission::FileEmission(FileEmission::CustomModule(Module::from_string("")))
    } else {
        // At least one Default inner type → Default
        Emission::FileEmission(FileEmission::Default)
    }
}
