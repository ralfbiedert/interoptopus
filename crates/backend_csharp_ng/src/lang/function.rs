use crate::lang::meta::Visibility;
use crate::lang::types::TypeIdCs;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Argument {
    pub name: String,
    pub ty: TypeIdCs,
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Signature {
    pub arguments: Vec<Argument>,
    pub rval: TypeIdCs,
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum FunctionKind {
    /// A forward function declaration without any C# body
    RustFunction,
    /// An overload with different parameters that essentially calls a `RustFunction`
    /// with the same name.
    RustOverload, // TODO, do we need much more fine-grained info about that?
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Overload {
    pub signature: Signature,
    pub kind: FunctionKind,
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Function {
    pub name: String,
    pub visibility: Visibility,
    pub overloads: Vec<Overload>,
}
