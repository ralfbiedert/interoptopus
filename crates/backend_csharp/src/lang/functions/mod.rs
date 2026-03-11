pub mod overload;

use crate::lang::TypeId;

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
    pub name: String,
    pub signature: Signature,
}
