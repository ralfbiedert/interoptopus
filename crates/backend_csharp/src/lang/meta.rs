/// A module identifier, e.g., `foo.bar`.
///
/// This does not have any semantic meaning beyond it being used to map
/// a module string (`foo.bar`) to a file name (e.g., `Interop.Foo.Bar.cs`)
/// at a later stage.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Module(String);

/// Items that should actually go to some file.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum FileEmission {
    /// This is a 'common' type (like Slice<u8>) that needs to be emitted in some interop file,
    /// is not a builtin, but not specific to any customer project.
    Common,
    /// The type should be placed in the given module / file.
    Module(Module),
}

/// Where an item definition should be placed in generated files.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Emission {
    /// This is a built-in type (e.g., `float`) and does not need to be emitted
    /// in generated interop code.
    Builtin,
    /// Items that should actually go to some file.
    FileEmission(FileEmission),
    /// The backend will _use_ the type as if it were auto-generated, but it is up to the user
    /// to actually provide it. Its definition will not be emitted.
    External,
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Visibility {
    Public,
    Private,
    Protected,
}
