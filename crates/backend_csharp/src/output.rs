#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum OutputKind {
    Csharp,
    Custom(String),
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Output {
    pub name: String,
    pub kind: OutputKind,
}
