use crate::inventory::{FunctionId, TypeId};
use crate::lang::meta::{Docs, Emission, Visibility};

pub trait FunctionInfo {
    fn id() -> FunctionId;
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Argument {
    pub name: String,
    pub ty: TypeId,
}

impl Argument {
    pub fn new(name: impl AsRef<str>, ty: TypeId) -> Self {
        Self { name: name.as_ref().to_string(), ty }
    }
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Signature {
    pub arguments: Vec<Argument>,
    pub rval: TypeId,
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Function {
    pub name: String,
    pub visibility: Visibility,
    pub docs: Docs,
    pub emission: Emission,
    pub signature: Signature,
}
