use crate::lang::TypeId;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum IntPtrHint {
    Read,
    ReadWrite,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum PointerKind {
    IntPtr(IntPtrHint),
    ByRef,
    ByOut,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Pointer {
    pub kind: PointerKind,
    pub target: TypeId,
}
