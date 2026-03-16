pub mod overload;

use crate::lang::TypeId;
use crate::lang::functions::overload::Overload;
use crate::lang::meta::Emission;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Argument {
    pub name: String,
    pub ty: TypeId,
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Signature {
    pub arguments: Vec<Argument>,
    pub rval: TypeId,
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Function {
    pub emission: Emission,
    pub name: String,
    pub signature: Signature,
    pub kind: FunctionKind,
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum FunctionKind {
    /// A base function with canonical (base) types, e.g., `IntPtr` only.
    Original,
    /// A function that calls a base function (or at least replaces parameters like `IntPtr` -> `ref uint`).
    Overload(Overload),
}
