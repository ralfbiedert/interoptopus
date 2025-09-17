/// The visibility of an item when written. Not all backends support all visibility levels.
#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Visibility {
    Public,
    Private,
}

/// Where an item definition should be placed in generated files.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Emission {
    /// This is a built-in type (e.g., `f32` <-> `float`) and does not need to be defined. Also
    /// used for "std" like builtins, `String` <-> `string`.
    Builtin,
    /// This is a 'common' type (like Slice<u8>) that needs to be emitted, is not a builtin,
    /// but not specific to any customer project.
    Common,
    /// The type should be placed in the given module / file. Backends decide how to handle this.
    Module(String),
    /// The backend will _use_ the type as if it were auto-generated, but it is up to the user
    /// to actually provide it.
    External,
}

/// Markdown generated from the `///` you put on Rust code.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
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
