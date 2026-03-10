use crate::lang::function::Signature;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DelegateKind {
    Class,
    Signature,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Delegate {
    pub kind: DelegateKind,
    pub signature: Signature,
}
